'use client'

import { useState, useEffect } from 'react'
import SearchingPanel, { SearchQuery } from '@/components/SearchingPanel'
import TrackCard, { Track } from '@/components/TrackCard'

const TrackList = ({ tracks }: { tracks: Track[] }) => {
  const [playingInfo, setPlayingInfo] = useState<{ id: number, time: number } | null>(null);

  const handlePlayPause = (id: number, isPlaying: boolean, ellapsedTime: number) => {
    if (isPlaying) {
      setPlayingInfo(null);
    } else {
      setPlayingInfo({ id: id, time: 0 });
    }
    // impl later
  }

  const handleDownload = (id: number) => {
    // impl later
  }

  return (
    <section className="w-full flex flex-col items-center">
      <p className="mb-4 text-muted-foreground md:text-base lg:max-w-2xl lg:text-lg">
        {tracks.length} results matched
      </p>
      <div className="w-full max-w-2xl grid grid-cols-1 gap-4">
        {tracks.map((track) => (
          <TrackCard
            key={track.id}
            track={track}
            isPlaying={playingInfo?.id === track.id}
            ellapsedTime={playingInfo?.id === track.id ? playingInfo.time : 0}
            onPlayPause={handlePlayPause}
            onDownload={handleDownload}
          />
        ))}
      </div>
    </section>
  )
}

const HomeBody = () => {
  const [tracks, setTracks] = useState<Track[]>([]);

  useEffect(() => {
    const fetchInitialTracks = async () => {
      try {
        const response = await fetch('http://localhost:3001/tracks');
        const data = await response.json();
        setTracks(data);
      } catch (error) {
        console.error("Failed to fetch initial tracks:", error);
      }
    };

    fetchInitialTracks();
  }, []);

  const handleSearch = async (query: SearchQuery) => {
    console.log('Search query:', query)
    const searchParams = new URLSearchParams();
    if (query.pattern) {
      searchParams.append('pattern', query.pattern);
    }
    if (query.order_by) {
      searchParams.append('order_by', query.order_by);
    }
    if (query.vibes) {
      query.vibes.forEach(vibe => searchParams.append('vibes', vibe));
    }

    try {
      const response = await fetch(`http://localhost:3001/tracks?${searchParams.toString()}`);
      const data = await response.json();
      setTracks(data);
    } catch (error) {
      console.error("Failed to fetch search results:", error);
    }
  }

  return (
    <section className="py-16">
      <div className="container mx-auto flex flex-col items-center gap-8 lg:px-16">
        <SearchingPanel onSearch={handleSearch} />
        <TrackList tracks={tracks}/>
      </div>
    </section>
  )
}

export default HomeBody