import React, { useEffect, useMemo, useRef, useState } from '@rbxts/react';
import { ContentProvider, Players } from '@rbxts/services';

type AvatarVariant = 'circle' | 'square' | 'rounded';
type AvatarType = 'headshot' | 'bust' | 'full';

const typeMap: Record<AvatarType, Enum.ThumbnailType> = {
  headshot: Enum.ThumbnailType.HeadShot,
  bust: Enum.ThumbnailType.AvatarBust,
  full: Enum.ThumbnailType.AvatarThumbnail,
};

const sizeMap: Record<number, Enum.ThumbnailSize> = {
  48: Enum.ThumbnailSize.Size48x48,
  60: Enum.ThumbnailSize.Size60x60,
  100: Enum.ThumbnailSize.Size100x100,
  150: Enum.ThumbnailSize.Size150x150,
  180: Enum.ThumbnailSize.Size180x180,
  420: Enum.ThumbnailSize.Size420x420,
};

interface AvatarProps extends Partial<JSX.IntrinsicElements['imagelabel']> {
  userId: number;
  dimension?: number;
  avatarType?: AvatarType;
  variant?: AvatarVariant;
  fallbackText?: string;
  border?: boolean;
}

export function Avatar({
  userId,
  dimension = 60,
  avatarType = 'headshot',
  variant = 'circle',
  fallbackText,
  border = false,
  ...rest
}: AvatarProps) {
  const avatarSheet = script.FindFirstChild('Avatar') as StyleSheet;

  const [image, setImage] = useState<string>('');
  const [loaded, setLoaded] = useState<boolean>(false);
  const imageRef = useRef<ImageButton>(undefined!);

  const corner = useMemo(() => {
    return variant === 'square'
      ? new UDim(0, 0)
      : variant === 'rounded'
      ? new UDim(0, math.floor(dimension / 6))
      : new UDim(1, 0);
  }, [variant, dimension]);

  useEffect(() => {
    let alive = true;

    const fetchThumb = () => {
      const [url] = Players.GetUserThumbnailAsync(
        userId,
        typeMap[avatarType],
        sizeMap[math.clamp(dimension, 48, 420) ?? Enum.ThumbnailSize.Size60x60]
      );

      ContentProvider.PreloadAsync([url]);

      if (alive) {
        setImage(url);
      }
    };

    fetchThumb();

    return () => {
      alive = false;
    };
  }, [userId, avatarType, dimension]);

  useEffect(() => {
    const img = imageRef.current;

    if (!img) return;

    const conn = img
      .GetPropertyChangedSignal('IsLoaded')
      .Connect(() => setLoaded(img.IsLoaded));

    return () => conn.Disconnect();
  }, []);

  return (
    <frame
      Size={UDim2.fromOffset(dimension, dimension)}
      BackgroundTransparency={1}
    >
      <uiaspectratioconstraint AspectRatio={1} />
      <imagelabel Image={image} Tag='Avatar' {...rest}>
        <uicorner CornerRadius={corner} />
        {border && <uistroke Thickness={1} Transparency={0.2} />}
        <stylelink StyleSheet={avatarSheet} key={'AvatarStyleLink'} />
      </imagelabel>
    </frame>
  );
}
