export type Language = (typeof languages)[number];

const languages = [
  {
    name: "Ruby",
    svgSrc: "ruby-original.svg",
    width: "82px",
    height: "66px"
  },
  {
    name: "Java",
    svgSrc: "java-plain-wordmark.svg",
    width: "82px",
    height: "66px"
  },
  {
    name: "Bash",
    svgSrc: "bash-original.svg",
    width: "82px",
    height: "66px"
  },
  {
    name: "C",
    svgSrc: "c-original.svg",
    width: "82px",
    height: "66px"
  },
  {
    name: "Fortran",
    svgSrc: "fortran-original.svg",
    width: "82px",
    height: "66px"
  },
  {
    name: "Rust",
    svgSrc: "rust-original.svg",
    width: "82px",
    height: "66px"
  }
] as const;

export default languages;
