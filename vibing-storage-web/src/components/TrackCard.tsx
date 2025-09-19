'use client'

import { useState, useRef, useEffect } from 'react'
import { Play, Pause, Download, ChevronDown, ChevronUp, Star } from 'lucide-react'

import { Card, CardContent } from '@/components/ui/card'
import { Badge } from '@/components/ui/badge'

// Define the Track interface based on the structure in your home page
export interface Track {
  id: number;
  url: string;
  title: string;
  author: string;
  image: string;
  genre: string;
  duration: number;
  vibes: { group_name: string, name: string }[];
  average_rating: number;
  download_count: number;
}

interface TrackCardProps {
  track: Track;
  isPlaying: boolean;
  ellapsedTime: number;
  onPlayPause: (id: number, isPlaying: boolean, ellapsedTime: number) => void;
  onDownload: (id: number) => void;
}

export default function TrackCard(
  {track, isPlaying, ellapsedTime, onPlayPause, onDownload }: TrackCardProps
) {
  const [isExpanded, setIsExpanded] = useState(false)
  const audioRef = useRef<HTMLAudioElement>(null)

  useEffect(() => {
    if (isPlaying) {
      audioRef.current?.play()
    } else {
      audioRef.current?.pause()
    }
  }, [isPlaying])

  const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL;

  const AUDIO_API = `${BACKEND_URL}/tracks/stream?track_id=${track.id.toString()}`;
  const DOWNLOAD_API = `${BACKEND_URL}/tracks/download?track_id=${track.id.toString()}`;

  return (
    <Card className="w-full p-4 transition-all duration-300">
      <audio ref={audioRef} src={AUDIO_API} />
      <div className="flex items-center justify-between gap-4">
        {/* Left Icon */}
        <div className="flex-shrink-0">
          <button 
            className="p-2 rounded-full hover:bg-muted" 
            aria-label={isPlaying ? "Pause track" : "Play track"}
            onClick={() => onPlayPause(track.id, isPlaying, ellapsedTime)}
          >
            {isPlaying ? (
              <Pause className="size-7 text-muted-foreground" />
            ) : (
              <Play className="size-7 text-muted-foreground" />
            )}
          </button>
        </div>

        {/* Middle Content */}
        <div className="flex-1 flex flex-col text-left">
          <h3 className="font-semibold text-lg">{track.title}</h3>
          <p className="text-sm text-muted-foreground">{track.author}</p>
          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="flex items-center text-sm text-blue-500 hover:underline mt-1"
          >
            {isExpanded ? 'Read less' : 'Read more'}
            {isExpanded ? <ChevronUp className="ml-1 size-4" /> : <ChevronDown className="ml-1 size-4" />}
          </button>
        </div>

        {/* Right Section (Rating and Download) */}
        <div className="flex items-center gap-6">
          {/* Rating Section */}
          <div className="flex items-center gap-1.5">
            <span className="font-bold text-sm">{track.average_rating.toFixed(1)}/5.0</span>
            <Star className="size-5 text-yellow-400 fill-yellow-400" />
          </div>

          {/* Download Section */}
          <div className="flex flex-col items-center gap-1">
            <a
              href={DOWNLOAD_API}
              target="_blank"
              rel="noopener noreferrer"
              download
              className="p-2 rounded-full hover:bg-muted"
              aria-label="Download track"
              onClick={() => onDownload(track.id)}
            >
              <Download className="size-6" />
            </a>
            <span className="text-xs font-mono text-muted-foreground">{track.download_count}</span>
          </div>
        </div>
      </div>

      {/* Expandable Content */}
      {isExpanded && (
        <CardContent className="pt-4 text-left border-t mt-4">
            <p className="text-sm"><span className="font-semibold">Genre:</span> {track.genre}</p>
            <p className="text-sm"><span className="font-semibold">Duration:</span> {track.duration}s</p>
            <div className="mt-2 flex flex-wrap gap-2">
                {track.vibes.map((tag) => (
                    <Badge key={tag.name} variant="secondary" className="capitalize">{tag.name}</Badge>
                ))}
            </div>
        </CardContent>
      )}
    </Card>
  )
}
