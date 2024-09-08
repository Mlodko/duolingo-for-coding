type CourseProgress = {
    course: number;
    unit: number;
    sector: number;
    level: number;
    task: number;
};

type Level = {
    level: number;
    XP: number;
};

type User = {
    loggedIn: boolean;

    id: string | undefined;
    username: string;
    passwordHash: string;
    email: string;
    phone: string;

    bio: string;
    friends: string[];

    level: Level;
    progress: CourseProgress;

    authToken: string | null;
};

export const currentUser: User = {
    loggedIn: false, 
    id: " ", 
    username: "",
    passwordHash: "",
    email: "",
    phone: "",

    bio: "", 
    friends: [""],

    level: {
        level: 0,
        XP: 0,
    },

    progress: {
        course: 0,
        unit: 0,
        sector: 0,
        level: 0,
        task: 0
    },

    authToken: " "
}