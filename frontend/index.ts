import axios from 'axios';

(async () => {
    const response = (await axios.get<ContentEntry>('http://localhost:4000/')).data;

    switch (response.content_metadata.type) {
        case ContentType.Music:
            console.log(`Artist: ${response.content_metadata.artist}\nLabel: ${response.content_metadata.label}\nGenre: ${response.content_metadata.genre}\nTracks: ${response.content_metadata.tracks}\n`);
            break;
        case ContentType.Television:
            console.log(`Director: ${response.content_metadata.director}\nProducer: ${response.content_metadata.producer}\nSeasons: ${response.content_metadata.seasons}\nEpisodes: ${response.content_metadata.episodes}\n`);
            break;
        case ContentType.Movie:
            console.log(`Director: ${response.content_metadata.director}\nProducer: ${response.content_metadata.producer}\nDuration Mins: ${response.content_metadata.duration_mins}\n`);
            break;
        default:
            exhaustive(response.content_metadata);
    }
})();

function exhaustive(x: never): never {
    throw new Error("Unhandled animal kind in the system");
}

type ContentEntry = {
    entry_id: number,
    content_url: string,
    content_metadata: Music | Television | Movie
}

enum ContentType {
    Music = "Music",
    Television = "Television",
    Movie = "Movie"
}

type Music = {
    type: ContentType.Music,
    artist: string,
    label: string,
    genre: string,
    tracks: number
}

type Television = {
    type: ContentType.Television,
    director: string,
    producer: string,
    seasons: number,
    episodes: number
}

type Movie = {
    type: ContentType.Movie,
    director: string,
    producer: string,
    duration_mins: number
}
