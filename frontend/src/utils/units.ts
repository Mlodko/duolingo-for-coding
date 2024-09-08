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
    description: "basic Java syntax",
    backgroundColor: "bg-pink-ish",
    textColor: "text-darker-purple",
    borderColor: "border-dark-purple",
    tiles: [
      {
        type: "book",
        description: "variables basics",
      },
      {
        type: "book",
        description: "object basics",
      },
      {
        type: "book",
        description: "methods basics",
      },
      { type: "book", description: "Class approach" },
      { type: "star", description: "unit 1 - review" },
    ],
  },
  {
    unitNumber: 2,
    description: "some skills yet to come...",
    backgroundColor: "bg-pink-ish",
    textColor: "text-darker-purple",
    borderColor: "border-dark-purple",
    tiles: [
      {
        type: "book",
        description: "some lesson",
      },
      {
        type: "book",
        description: "some lesson",
      },
      {
        type: "book",
        description: "some lesson",
      },
      { type: "book", description: "some lesson" },
      { type: "dumbbell", description: "some lesson" },
      {
        type: "book",
        description: "some lesson",
      },
      {
        type: "book",
        description: "some lesson",
      },
      {
        type: "book",
        description: "some lesson",
      },
      { type: "book", description: "some lesson" },
      { type: "book", description: "some lesson" },
      { type: "star", description: "unit 2 - review" },
    ],
  },
];
