export type SizeUnit = "B" | "KB" | "MB" | "GB";
export type PathConfig = {
    path:string,
    config:{
        include:string,
        exclude:string,
        size_extend:[number,number]
    }
}