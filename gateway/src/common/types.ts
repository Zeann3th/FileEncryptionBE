export interface UserInterface {
    sub: string;
    username: string;
    role: "ADMIN" | "USER";
    email: string;
}