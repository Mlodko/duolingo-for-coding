import bcryptjs from "bcryptjs"
import { currentUser } from "./userData";

const SERVER: string = `http://127.0.0.1:8080`;
const ENDP_USER: string = `/user`;
const ENDP_TEST: string = `/test`;
const ENDP_LOGIN: string = `/user/login`;
const ENDP_REGISTER: string = `/user/register`;
const ENDP_LOGOUT: string = `/user/logout`;

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

    const response = await fetch(SERVER + ENDP_TEST, {
        method: "GET",
        mode:"cors"
    });
    
    console.log("we got " + (response).status + " in ServerTest");
}

export async function UserLogIn (Username: string, Password: string) {
    try {
        //const salt:string = await bcryptjs.genSalt(3);
        const passwordHash: string = Password; //await bcryptjs.hash(Password, salt);

        const prepData = {
            username: Username,
            password: passwordHash,
        }

        //console.log("we sendin " + Username + ", " + passwordHash);

        const response = await fetch(SERVER + ENDP_LOGIN, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json' 
            },
            body: JSON.stringify(prepData),
            mode:"cors"
        });

        if ((response).status === 200) {        
            currentUser.username = Username;
            currentUser.passwordHash = passwordHash;
            currentUser.authToken = (response).headers.get("Authorization");
            currentUser.id = await response.text();
            currentUser.loggedIn = true;
            GetCurrentUserData();
            
            console.log("we logged in! id: " + currentUser.id + " \nand the authtoken: " + currentUser.authToken);
            return true;
        }
        else {
            console.log("after the fetch, we got " + (response).status.toString() + " in UserLogIn");
            console.log("data sent - username: " + Username + " | password: " + passwordHash);
            return false;
        }
    } catch (error) {
        console.log("error in UserLogIn: " + error);
        return false;
    }

    return false;
}


export async function UserRegister (Username: string, Password: string, email: string | null, phone: string | null) {
    try {
        //const salt: string = await bcryptjs.genSalt(3);
        const passwordHash: string = Password; //await bcryptjs.hash(Password, salt);
        
        if (email === null)
            email = "";

        if (phone === null)
            phone = "";

        const prepData = {
            username: Username,
            password: passwordHash,
            email: email,
            phone: phone
        };

        const response = await fetch(SERVER + ENDP_REGISTER, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(prepData),
            mode: "cors"
        });

        if ((response).status === 201)
        {
            console.log("user created!");
            UserLogIn(Username, passwordHash);
            return true;
        }
        else
        {
            console.log("after the fetch, we got " + (response).status.toString() + " in UserRegister");
            console.log("data sent - username: " + Username + " | password: " + passwordHash + " | email: " + email! + " | phone: " + phone!);
            return false;
        }
    } catch (error) {
        console.log("error in UserRegister: " + error);
    }

    return false;
}


/* Makes use of global "currentUser", so no arguments here. Requires the user to be already logged in! */
export async function GetCurrentUserData() {
    try {
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

            const response = await fetch(SERVER + ENDP_USER + `/` + currentUser.id, {
                method: "GET",
                headers: {
                    'Content-Type': 'application/json',
                    'Authorization': currentUser.authToken!
                },
                mode:"cors"
            });    

            if ((response).status === 200) {
                const respTxt = (await (response).text());
                console.log("respTxt: " + respTxt);
                const respJson = JSON.parse(respTxt);
                currentUser.username = respJson.username;
                currentUser.email = respJson.email;
                currentUser.phone = (respJson.phone === null ? "" : respJson.phone);
                currentUser.bio = respJson.bio;
                currentUser.friends = respJson.friends;
                currentUser.level = respJson.level;
                currentUser.progress = respJson.progress;
                
                console.log("user's data properply fetched from the server");
                return true;
            }
            else {
                console.log("after the fetch, we got " + (response).status.toString() + " in GetCurrentUserData");
                console.log("used creds - id: " + currentUser.id + " \nauth-token: " + currentUser.authToken);    
                return false;
            }
        }
        else {
            console.log("improper call of GetCurrentUserData");
            return false;
        }
    } catch (error) {
        console.log("error in GetCurrentUserData: " + error);
    }
}

// Makes use of global "currentUser", so no arguments here. Requires the user to be already logged in!
export async function UserLogOut() {
    try {

        if (!currentUser.loggedIn) {
            console.log("bro wtf, ur not even logged in, and ya want to log out");
            return false;
        }

        const response = await fetch(SERVER + ENDP_LOGOUT, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json',
                'Authorization': currentUser.authToken!
            },
            mode: "cors"
        });

        if ((response).status === 200) {
            currentUser.authToken = "";
            currentUser.bio = "";
            currentUser.email = "";
            currentUser.friends = [];
            currentUser.id = "";
            currentUser.level = { level: 0, XP: 0};
            currentUser.loggedIn = false;
            currentUser.passwordHash = "";
            currentUser.phone = "";
            currentUser.username = "";
            currentUser.progress = {
                course: "",
                unit: "",
                sector: 0,
                level: 0,
                task: 0
            };

            console.log("user properly logged out");
            
            return true;
        }
        else 
        {
            console.log("we got " + response.status + " in UserLogOut");
        }
    } catch (error) {
        console.log("error in UserLogOut: " + error);
    }

    return false;
}

// TODO
export async function UserDataUpdate() {

} 