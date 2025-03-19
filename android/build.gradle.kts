import com.android.build.gradle.internal.cxx.configure.gradleLocalProperties

// Top-level build file where you can add configuration options common to all sub-projects/modules.
@Suppress("DSL_SCOPE_VIOLATION") // TODO: Remove once KTIJ-19369 is fixed
plugins {
    alias(libs.plugins.kotlinAndroid) apply false
    alias(libs.plugins.androidLibrary) apply false
    alias(libs.plugins.cargoNdk) apply false
    `maven-publish`
}

subprojects {
    apply(plugin = "maven-publish")
    configure<PublishingExtension> {
        repositories {
            maven {
                name = "GitHubPackages"
                url = uri("https://maven.pkg.github.com/Proximie/janus-mobile-sdk")
                credentials {
                    val properties = gradleLocalProperties(rootDir)
                    username = properties.getProperty("gpr.user") ?: System.getenv("GITHUB_USERNAME")
                    password = properties.getProperty("gpr.key") ?: System.getenv("GITHUB_TOKEN")
                }
            }
        }
    }
}

allprojects {
    group = GradleConfigs.baseNamespace
    version = GradleConfigs.packageVersion
}
