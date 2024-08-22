import type { Language } from "~/utils/languages";

export const Flag = ({
  language,
  width = 75,
}: {
  language: Language;
  width?: number;
}) => {
  const height = width * (19.3171 / 24);
  return (
    <svg viewBox="" style={{ height, width }}>
      <image
        height={height}
        href={language.svgSrc}
        width={width}
      ></image>
    </svg>
  );
};
