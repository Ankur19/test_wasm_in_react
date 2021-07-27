data class Disk(
    val CD: List<Song>,
    val TIME: Int
)

data class Song(
    val TITLE: String,
    val ARTIST: String,
    val COUNTRY: String,
    val COMPANY: String,
    val PRICE: String,
    val YEAR: String
)