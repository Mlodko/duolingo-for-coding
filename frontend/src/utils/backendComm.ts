import bcryptjs from "bcryptjs"
import { currentUser } from "./userData";

const SERVER: string = `http://127.0.0.1:8080`;
const ENDP_USER: string = `/user`;
const ENDP_TEST: string = `/test`;
const ENDP_LOGIN: string = `/user/login`;
const ENDP_REGISTER: string = `/user/register`;

function PrintCurrentProgress() {
    console.log(
        `course:` + currentUser.progress.course +
        `\nunit:` + currentUser.progress.unit +
        `\nsector:` + currentUser.progress.sector + 
        `\nlevel:` + currentUser.progress.level +
        `\ntask:` + currentUser.progress.task +
    `\n`);
}

function PrintCurrentLevel() {
    console.log(
        `level:` + currentUser.level.level + 
        `\nXP:` + currentUser.level.XP +
        `\n`
    );
}

export function PrintCurrentUser() {
    console.log(    
        `loggedIn:` + currentUser.loggedIn +
        `\nid:` + currentUser.id + 
        `\nusername:` + currentUser.username +
        `\npasswordHash:` + currentUser.passwordHash +
        `\nemail:` + currentUser.email + 
        `\nphone:` + currentUser.phone +
        `\nbio:` + currentUser.bio + 
        `\nfriends:` + currentUser.friends +
        `\nlevel:\n` + PrintCurrentLevel() +
        `\nprogress\n:` + PrintCurrentProgress() + 
        `\nauthToken:` + currentUser.authToken + `\n`
    );
}

export async function ServerTest () {

    const resp = fetch(SERVER + ENDP_TEST, {
        method: "GET",
        mode:"no-cors",
    });

    console.log("we got " + (await resp).status.toString() + " in ServerTest");
}

export async function UserLogIn (Username: string, Password: string) {
    const salt:string = await bcryptjs.genSalt(3);
    const passwordHash: string = Password; //await bcryptjs.hash(Password, salt);

    //console.log("we sendin " + Username + ", " + passwordHash);

    const response = fetch(SERVER + ENDP_LOGIN, {
        method: "POST",
        headers: {
            'Content-Type': 'application/json' 
        },
        body: `{"username":"` + Username + `","password_hash":"` + passwordHash + `"}`,
        mode:"no-cors"
    });

    if ((await response).status === 200) {        
        currentUser.username = Username;
        currentUser.passwordHash = passwordHash;
        currentUser.authToken = (await response).headers.get("Authorization");
        currentUser.id = await (await response).text();
        currentUser.loggedIn = true;
        GetCurrentUserData();
        
        console.log("we logged in! id: " + currentUser.id + " \nand the authtoken: " + currentUser.authToken);
    }
    else {
        console.log("after the fetch, we got " + (await response).status.toString() + " in UserLogIn");
        console.log("data sent - username: " + Username + " | passwordHash: " + passwordHash);
    }
}


export async function UserRegister (Username: string, Password: string, email: string | null, phone: string | null) {
    const salt: string = await bcryptjs.genSalt(3);
    const passwordHash: string = Password; //await bcryptjs.hash(Password, salt);
    
    if (email === null)
        email = "";

    if (phone === null)
        phone = "";

    const response = fetch(SERVER + ENDP_REGISTER, {
        method: "POST",
        headers: {
            'Content-Type': 'application/json' 
        },
        body: `{"username":"` + Username + `","password_hash":"` + passwordHash + `","email":"` + email! + `","phone":"` + phone! + `"}`,
        mode:"no-cors"
    });

    if ((await response).status === 201)
    {
        console.log("user created!");
        UserLogIn(Username, passwordHash);
    }
    else
    {
        console.log("after the fetch, we got " + (await response).status.toString() + " in UserRegister");
        console.log("data sent - username: " + Username + " | passwordHash: " + passwordHash + " | email: " + email! + " | phone: " + phone!);
    }
}


/* Makes use of global "currentUser", so no arguments here. Requires the user to be already logged in! */
export async function GetCurrentUserData() {
    if (currentUser.loggedIn) {

        if (currentUser.authToken === null)
        {
            console.log("currentUser.authToken === null -> empty string");
            currentUser.authToken = "";
        }

        if (currentUser.id === undefined)
        { 
            console.log("currentUser.id === undefined -> empty string");
            currentUser.id = "";
        }

        const response = fetch(SERVER + ENDP_USER + `/` + currentUser.id, {
            method: "GET",
            headers: {
                'Content-Type': 'application/json', 
                'Authorization': currentUser.authToken!
            },
            mode:"no-cors"
        });    

        if ((await response).status === 200) {
            const respJson = JSON.parse((await (await response).text()));
            currentUser.username = respJson.username;
            currentUser.email = respJson.email;
            currentUser.phone = respJson.phone;
            currentUser.bio = respJson.bio;
            currentUser.friends = respJson.friends;
            currentUser.level = respJson.level;
            currentUser.progress = respJson.progress;
            
            console.log("user's data properply fetched from the server");
        }
        else {
            console.log("after the fetch, we got " + (await response).status.toString() + " in GetCurrentUserData");
            console.log("used creds - id: " + currentUser.id + " \nauth-token: " + currentUser.authToken);    
        }
    }
    else {
        console.log("improper call of GetCurrentUserData");
    }
}