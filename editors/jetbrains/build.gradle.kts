plugins {
  id("java")
  id("org.jetbrains.intellij") version "1.17.0"
  id("org.jetbrains.kotlin.jvm") version "1.9.20"
}

group = "com.wayfinder"
version = "1.0.0"

repositories {
  mavenCentral()
}

dependencies {
  testImplementation("junit:junit:4.13.2")
}

intellij {
  version.set("2023.2")
  type.set("IC") // IntelliJ Community Edition
  updateSinceUntilBuild.set(true)

  plugins.set(listOf(
    "com.intellij.java",
    "org.jetbrains.kotlin",
    "com.intellij.database"
  ))
}

tasks {
  withType<JavaCompile> {
    sourceCompatibility = "17"
    targetCompatibility = "17"
  }

  withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    kotlinOptions.jvmTarget = "17"
  }

  patchPluginXml {
    sinceBuild.set("232")
    untilBuild.set("241.*")
  }

  signPlugin {
    certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
    privateKey.set(System.getenv("PRIVATE_KEY"))
    password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
  }

  publishPlugin {
    token.set(System.getenv("PUBLISH_TOKEN"))
  }
}
