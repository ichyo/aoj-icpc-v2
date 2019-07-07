export interface Problem {
    id: number,
    point: number,
    title: string,
    source: string,
    solutions: number,
    url: string,
    stars: number,
    status: string,
    year: number,
    source_type: string,
}

export interface User {
    id: string,
    solutions: Array<number>,
}
