import com.android.build.gradle.internal.tasks.factory.dependsOn
import org.gradle.configurationcache.extensions.capitalized

plugins {
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinAndroid)
    alias(libs.plugins.cargoNdk)
    `maven-publish`
}

android {
    namespace = GradleConfigs.subNamespace("janus")
    compileSdk = GradleConfigs.compileSdk
    ndkVersion = GradleConfigs.ndkVersion

    defaultConfig {
        minSdk = GradleConfigs.minSdk
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
    }

    publishing {
        singleVariant("release") {
            withSourcesJar()
            withJavadocJar()
        }
    }
}

dependencies {
    implementation(libs.jna)
    implementation(libs.core.ktx)
}

cargoNdk {
    module = ".."
    librariesNames = arrayListOf("libjanus_gateway.so")
}

afterEvaluate {
    android.libraryVariants.all { variant ->
        val outDir = "${layout.buildDirectory}/generated/source/uniffi/${variant.name}/java"

        val generateBindings = tasks.register(
            name = "generate${variant.name.capitalized()}UniFFIBindings",
            type = Exec::class
        ) {
            workingDir = file("../..")
            commandLine(
                "cargo", "run", "-p", "uniffi-bindgen", "generate",
                "--library", "./android/janus/src/main/jniLibs/arm64-v8a/libjanus_gateway.so",
                "--language", "kotlin",
                "--out-dir", outDir
            )
            dependsOn("buildCargoNdk${variant.name.capitalized()}")
        }

        val copyBindings = tasks.register(
            name = "copy${variant.name.capitalized()}UniFFIBindings",
            type = Exec::class
        ) {
            workingDir = file("../..")
            commandLine("cp", "-r", outDir, "${projectDir}/src/main/")
            dependsOn(generateBindings)
        }
        variant.javaCompileProvider.dependsOn(copyBindings)
        tasks.named("compile${variant.name.capitalized()}Kotlin") { dependsOn(generateBindings) }
        tasks.named("connectedDebugAndroidTest").configure { dependsOn(generateBindings) }
        true
    }
}

configure<PublishingExtension> {
    publications {
        create<MavenPublication>("${project.name}-release") {
            artifactId = "janus"

            afterEvaluate {
                from(components["release"])
            }
        }
    }
}
