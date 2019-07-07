import React from 'react';
import { Problem, User } from '../model';
import { number } from 'prop-types';

interface Props {
    problems: Array<Problem>,
    user: User | null,
}

const constructPointMap = (problems: Array<Problem>) => {
    const totalMap = new Map<number, number>();
    for (const p of problems) {
        if (p.point != 0) {
            totalMap.set(p.point, (totalMap.get(p.point) || 0) + 1);
        }
    }
    return totalMap;
}

const constructSolvedProblems = (problems: Array<Problem>, user: User) => {
    const solvedId = user ? new Set(user.solutions) : new Set();
    return problems.filter(p => solvedId.has(p.id));
}

const totalSolutions = (pointMap: Map<number, number>) => {
    let total = 0;
    pointMap.forEach((v) => {
        total += v;
    });
    return total;
}

const totalPoints = (pointMap: Map<number, number>) => {
    let total = 0;
    pointMap.forEach((v, k) => {
        total += k * v;
    });
    return total;
}

const PointSummary: React.FC<Props> = ({ problems, user }: Props) => {
    const points = Array.from(new Set(problems.map(p => p.point)
        .filter(p => p != 0)))

    const totalMap = constructPointMap(problems);
    const userMap = user ? constructPointMap(constructSolvedProblems(problems, user)) : new Map();

    return (
        <table className="table table-sm">
            <thead>
                <tr>
                    <th>ID</th>
                    <th>POINTS</th>
                    <th>SOLVED</th>
                    {
                        points.map(p =>
                            <th>{p}</th>
                        )
                    }
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th>TOTAL</th>
                    <th>{totalPoints(totalMap)}</th>
                    <th>{totalSolutions(totalMap)}</th>
                    {
                        points.map(p =>
                            <th>{totalMap.get(p) || 0}</th>
                        )
                    }
                </tr>
                {
                    user
                        ? <tr>
                            <td>{user.id}</td>
                            <td>{totalPoints(userMap)}</td>
                            <td>{totalSolutions(userMap)}</td>
                            {
                                points.map(p =>
                                    <td>{userMap.get(p) || 0}</td>
                                )
                            }
                        </tr>
                        : null
                }
            </tbody>
        </table>
    );
};

export default PointSummary;

