object GradleConfigs {
    const val compileSdk = 34
    const val minSdk = 26
    const val ndkVersion = "23.1.7779620"
    const val baseNamespace = "com.proximie"
    const val packageVersion = "0.1.1"

    fun subNamespace(sub: String) = "$baseNamespace.$sub"
}
