import { experimentalTernaries } from "prettier.config.cjs";

export enum TaskType { MultiChoice, Open, Construct };

export type AnswerContent = {
    Type: TaskType;
    Data: string | string[];
};

export type Answer = {
    TaskID: string;
    UserID: string;

    content: AnswerContent;
};

export type TaskContent = {
    Question: string;
    Data: null | string[];
};

export type Task = {
    ID: string;
    Title: string;
    Type: TaskType;

    Content: TaskContent;
};

export type Result = {
    ifCorrect: boolean;
    explanation: null | string;
};


export const TestTask: Task = {
    ID: "test2137",
    Title: "test",
    Type: TaskType.Construct,

    Content: {
        Question: "dżem dobry lodzicz",
        Data: ["dobry", "no", "UwU", "dżem", "helo"]
    },
};

export const TestAnswer: Answer = {
    TaskID: "test2137",
    UserID: "95920d77-0cf1-4251-ad67-a1a48c548981",

    content: {
        Type: TestTask.Type,
        Data: ["no", "dżem", "dobry"]
    }
};

export const CurrentTask: Task = {
    ID: "",
    Title: "",
    Type: TaskType.Open,

    Content: {
        Question: "",
        Data: null
    }
};

export const CurrentTaskAnswer: Answer = {
    TaskID: "",
    UserID: "",

    content: {
        Type: CurrentTask.Type,
        Data: ""
    }
};

export const CurrentResult: Result = {
    ifCorrect: false,
    explanation: ""
};

export const TasksDone: string[] = ["f2d7a6f58d8c41eb9bd2726306620065"];

export const TasksFailed: string[] = [];

export const CorrectAnswer = {
    ID: ""
};