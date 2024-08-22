export type Unit = {
  unitNumber: number;
  description: string;
  backgroundColor: `bg-${string}`;
  textColor: `text-${string}`;
  borderColor: `border-${string}`;
  tiles: Tile[];
};

/*

    Querying the database here? 
    -> static, non user-dependent?
    -> how tf do u do it in reacc?

*/

export type Tile =
  | {
      type: "star" | "dumbbell" | "book" | "trophy" | "fast-forward";
      description: string;
    }
  | { type: "treasure" };

export type TileType = Tile["type"];

export const units: readonly Unit[] = [
  {
    unitNumber: 1,
    description: "adbwebrfgb",
    backgroundColor: "bg-pink-ish",
    textColor: "text-darker-purple",
    borderColor: "border-dark-purple",
    tiles: [
      {
        type: "book",
        description: "fsdjv",
      },
      {
        type: "book",
        description: "fbuyewsvjdn",
      },
      {
        type: "book",
        description: "fyudsjnv",
      },
      { type: "book", description: "dysbhcener" },
      { type: "star", description: "hbfdegyvhu" },
    ],
  },
  {
    unitNumber: 2,
    description: "fydtgsh",
    backgroundColor: "bg-pink-ish",
    textColor: "text-darker-purple",
    borderColor: "border-dark-purple",
    tiles: [
      {
        type: "book",
        description: "fsdjv",
      },
      {
        type: "book",
        description: "fbuyewsvjdn",
      },
      {
        type: "book",
        description: "fyudsjnv",
      },
      { type: "book", description: "dysbhcener" },
      { type: "dumbbell", description: "hbfdegyvhu" },
      {
        type: "book",
        description: "fsdjv",
      },
      {
        type: "book",
        description: "fbuyewsvjdn",
      },
      {
        type: "book",
        description: "fyudsjnv",
      },
      { type: "book", description: "dysbhcener" },
      { type: "book", description: "hbfdegyvhu" },
      { type: "star", description: "Unit 2 review" },
    ],
  },
];
