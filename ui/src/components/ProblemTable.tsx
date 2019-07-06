import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck, faCheckCircle, faStar } from '@fortawesome/free-solid-svg-icons'
import React from 'react';
import { Problem, User } from '../model';

interface Props {
    problems: Array<Problem>,
    user: User | null,
    solutions: Map<string, number>,
    handleSort: (comparator: (a: Problem, b: Problem) => number) => void,
}

interface RowProps {
    problem: Problem,
    solved: boolean,
    solution: number,
}

const ProblemRow: React.FC<RowProps> = ({ problem, solved, solution }) => {
    return (
        <tr key={problem.id}>
            <td className="text-center text-success">
                {solved ? <FontAwesomeIcon icon={faCheckCircle} /> : null}
            </td>
            <td className="text-center">{problem.point}</td>
            <td><a href={problem.url} target="_blank" rel="noopener noreferrer">{problem.title}</a></td>
            <td>{problem.source}</td>
            <td className="text-center">{problem.stars}</td>
            <td>{solution}</td>
        </tr>
    );
}

const ProblemTable: React.FC<Props> = ({ problems, user, solutions, handleSort }) => {
    const user_solutions = new Set(user ? user.solutions : []);
    return (
        <table className="table table-sm">
            <thead>
                <tr>
                    <th><FontAwesomeIcon icon={faCheck} /></th>
                    <th scope="col" className="text-center">
                        <a href="#" onClick={() => handleSort((a, b) => a.point - b.point)}>
                            Point
                        </a>
                    </th>
                    <th scope="col">
                        <a href="#" onClick={() => handleSort((a, b) => a.title.localeCompare(b.title))}>
                            Title
                        </a>
                    </th>
                    <th scope="col">
                        <a href="#" onClick={() => handleSort((a, b) => a.source.localeCompare(b.source))}>
                            Source
                        </a>
                    </th>
                    <th scope="col" className="text-center"><FontAwesomeIcon icon={faStar} /></th>
                    <th scope="col">
                        <a href="#" onClick={() => handleSort((a, b) =>
                            (solutions.get(b.id.toString()) || 0) -
                            (solutions.get(a.id.toString()) || 0))}>
                            Solutions
                        </a>
                    </th>
                </tr>
            </thead>
            <tbody>
                {problems.map(p => {
                    const solved = user_solutions.has(p.id);
                    return <ProblemRow
                        problem={p}
                        solved={solved}
                        solution={solutions.get(p.id.toString()) || 0}
                    />
                })}
            </tbody>
        </table>
    );
}

export default ProblemTable;
