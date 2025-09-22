import { Slider } from "@/components/ui/slider";
import React from "react";

interface VolumeSliderProps {
  volume: number; // 0 to 100
  onVolumeChange: (newVolume: number) => void;
}

export function VolumeSlider({ volume, onVolumeChange }: VolumeSliderProps) {
  return (
    <div className="w-[200px]">
      <Slider
        defaultValue={[volume]}
        max={100}
        step={1}
        onValueChange={(value) => onVolumeChange(value[0])}
        className="w-full"
      />
    </div>
  );
}
