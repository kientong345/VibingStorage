import SearchingPanel from '@/components/SearchingPanel'
import TrackCard from '@/components/TrackCard'

const tracks = [
  {
    id: 1,
    url: "https://example.com/music/1",
    title: 'MorningRain',
    author: 'Terraria',
    genre: 'house',
    duration: 100,
    tags: ['morning', 'rainy'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
  {
    id: 2,
    url: "https://example.com/music/2",
    title: 'Rain',
    author: 'Terraria',
    genre: 'house',
    duration: 100,
    tags: ['rainy'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
  {
    id: 3,
    url: "https://example.com/music/3",
    title: 'TownNight',
    author: 'Terraria',
    genre: 'house',
    duration: 100,
    tags: ['evening', 'night'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
  {
    id: 4,
    url: "https://example.com/music/4",
    title: 'Ocean',
    author: 'Terraria',
    genre: 'house',
    duration: 100,
    tags: ['summer'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
  {
    id: 5,
    url: "https://example.com/music/5",
    title: 'Summertime',
    author: 'cinemon',
    genre: 'house',
    duration: 100,
    tags: ['sunny', 'summer'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
  {
    id: 6,
    url: "https://example.com/music/6",
    title: 'GloriousMorning',
    author: 'Waterflame',
    genre: 'house',
    duration: 100,
    tags: ['morning'],
    image: '',
    average_rating: 0.00,
    download_count: 0
  },
];

const HomeBody = () => {
  return (
    <section className="py-32">
      <div className="container mx-auto flex flex-col items-center gap-10 lg:px-16">
        <SearchingPanel />
        <p className="mb-8 text-muted-foreground md:text-base lg:max-w-2xl lg:text-lg">
          there are {tracks.length} tracks that matched your filter
        </p>
        <div className="w-full max-w-2xl grid grid-cols-1 gap-4">
          {tracks.map((track) => (
            <TrackCard key={track.id} track={{
              ...track,
              id: track.id.toString(),
              duration: track.duration.toString(),
            }}/>
          ))}
        </div>
      </div>
    </section>
  )
}

export default HomeBody