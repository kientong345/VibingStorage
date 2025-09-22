'use client'

import { useState, useEffect } from 'react'
import SearchingPanel, { SearchQuery } from '@/components/SearchingPanel'
import TrackCard, { Track } from '@/components/TrackCard'
import { VolumeSlider } from '@/components/VolumeSlider'

const PAGE_SIZE = 10;

interface TrackListProps {
  tracks: Track[];
  currentVolume: number;
}

const TrackList = ( { tracks, currentVolume }: TrackListProps ) => {
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
            currentVolume={currentVolume}
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
  const [currentVolume, setCurentVolume] = useState(50);
  const [currentPage, setCurrentPage] = useState(1);

  useEffect(() => {
    const fetchInitialTracks = async () => {
      try {
        const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL;
        const response = await fetch(`${BACKEND_URL}/tracks?page=1&size=${PAGE_SIZE}`);
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
      const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL;
      const response = await fetch(`${BACKEND_URL}/tracks?${searchParams.toString()}`);
      const data = await response.json();
      setTracks(data);
    } catch (error) {
      console.error("Failed to fetch search results:", error);
    }
  }

  const handleVolumeChange = (newVolume: number) => {
    setCurentVolume(newVolume);
  }

  return (
    <section className="py-16">
      <div className="container mx-auto flex flex-col items-center gap-8 lg:px-16">
        <SearchingPanel
          currentPage={currentPage}
          pageSize={PAGE_SIZE}
          onSearch={handleSearch} />
        <TrackList tracks={tracks} currentVolume={currentVolume}/>
        <div className="fixed bottom-25 right-4 z-50">
          <VolumeSlider volume={currentVolume} onVolumeChange={handleVolumeChange} />
        </div>
      </div>
    </section>
  )
}

export default HomeBody