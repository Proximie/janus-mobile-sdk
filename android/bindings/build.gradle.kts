import com.android.build.gradle.internal.tasks.factory.dependsOn
import org.gradle.configurationcache.extensions.capitalized

plugins {
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.kotlinAndroid)
    alias(libs.plugins.cargoNdk)
    `maven-publish`
}

android {
    namespace = GradleConfigs.subNamespace("bindings")
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
        val generateBindings = tasks.register(
            name = "generate${variant.name.capitalized()}UniFFIBindings",
            type = Exec::class
        ) {
            workingDir = file("../..")
            val outDir = "${buildDir}/generated/source/uniffi/${variant.name}/java"
            commandLine(
                "cargo", "run", "-p", "uniffi-bindgen", "generate",
                "--library", "./android/bindings/src/main/jniLibs/arm64-v8a/libjanus_gateway.so",
                "--language", "kotlin",
                "--out-dir", outDir
            )
            // For some reason this doesn't work on a fresh build, we should comment it, run it,
            // uncomment, then run it again
            commandLine("cp", "-r", outDir, "${projectDir}/src/main/")
            dependsOn("buildCargoNdk${variant.name.capitalized()}")
        }
        variant.javaCompileProvider.dependsOn(generateBindings)
        tasks.named("compile${variant.name.capitalized()}Kotlin") { dependsOn(generateBindings) }
        tasks.named("connectedDebugAndroidTest").configure { dependsOn(generateBindings) }
        true
    }
}

configure<PublishingExtension> {
    publications {
        create<MavenPublication>("${project.name}-release") {
            artifactId = "bindings"

            afterEvaluate {
                from(components["release"])
            }
        }
    }
}
