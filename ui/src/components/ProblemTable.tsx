import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck, faCheckCircle, faStar } from '@fortawesome/free-solid-svg-icons'
import React from 'react';
import { Problem, User } from '../model';

interface Props {
    problems: Array<Problem>,
    user: User | null
}

interface RowProps {
    problem: Problem,
    solved: boolean,
}

const ProblemRow: React.FC<RowProps> = ({ problem, solved }) => {
    return (
        <tr key={problem.id}>
            <td className="text-center text-success">
                {solved ? <FontAwesomeIcon icon={faCheckCircle} /> : null}
            </td>
            <td className="text-center">{problem.point}</td>
            <td><a href={problem.url} target="_blank" rel="noopener noreferrer">{problem.title}</a></td>
            <td>{problem.source}</td>
            <td className="text-center">{problem.stars}</td>
            <td>{problem.solutions}</td>
        </tr>
    );
}

const ProblemTable: React.FC<Props> = ({ problems, user }) => {
    const solutions = new Set(user ? user.solutions : []);
    return (
        <table className="table table-sm">
            <thead>
                <tr>
                    <th><FontAwesomeIcon icon={faCheck} /></th>
                    <th scope="col" className="text-center">Point</th>
                    <th scope="col">Title</th>
                    <th scope="col">Source</th>
                    <th scope="col" className="text-center"><FontAwesomeIcon icon={faStar} /></th>
                    <th scope="col">Solutions</th>
                </tr>
            </thead>
            <tbody>
                {problems.map(p => {
                    const solved = solutions.has(p.id);
                    return <ProblemRow problem={p} solved={solved} />
                })}
            </tbody>
        </table>
    );
}

export default ProblemTable;