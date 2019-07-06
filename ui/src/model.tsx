export interface Problem {
    id: number,
    point: number,
    title: string,
    source: string,
    solutions: number,
    url: string,
    stars: number,
    status: string,
}

export interface User {
    solutions: Array<number>,
}
