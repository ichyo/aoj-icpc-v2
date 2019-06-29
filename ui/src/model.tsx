export interface Problem {
    id: number,
    point: number,
    title: string,
    source: string,
    solutions: number,
    url: string,
    stars: number,
}

export interface User {
    solutions: Array<number>,
}
