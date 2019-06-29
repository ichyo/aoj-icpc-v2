import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck, faCheckCircle, faStar } from '@fortawesome/free-solid-svg-icons'

interface Problem {
    id: number,
    point: number,
    title: string,
    source: string,
    solutions: number,
    url: string,
    stars: number,
}

interface User {
    solutions: Array<number>,
}

interface Props {
    problems: Array<Problem>,
    user: User | null,
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
                    return (
                        <tr key={p.id}>
                            <td className="text-center text-success">
                                {solved ? <FontAwesomeIcon icon={faCheckCircle} /> : null}
                            </td>
                            <td className="text-center">{p.point}</td>
                            <td><a href={p.url} target="_blank" rel="noopener noreferrer">{p.title}</a></td>
                            <td>{p.source}</td>
                            <td className="text-center">{p.stars}</td>
                            <td>{p.solutions}</td>
                        </tr>
                    );
                })}
            </tbody>
        </table>
    );
}

export default ProblemTable;