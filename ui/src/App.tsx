import React, { useState, useEffect } from 'react';
import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheckCircle, faStar } from '@fortawesome/free-solid-svg-icons'

interface Problem {
  point: number,
  title: string,
  source: string,
  solutions: number,
  url: string,
  stars: number,
}

interface TableProps {
  problems: Array<Problem>,
}

const Form: React.FC = () => {
  return (
    <form className="form-inline mb-3 mt-3">
      <input type="text" id="aoj-id" className="form-control mr-2 col-4 col-md-3" placeholder="AOJ ID" />
      <button type="submit" className="btn btn-primary">Update</button>
    </form>
  );
}

const Table: React.FC<TableProps> = ({ problems }) => {
  return (
    <table className="table table-sm">
      <thead>
        <tr>
          <th></th>
          <th scope="col" className="text-center">Point</th>
          <th scope="col">Title</th>
          <th scope="col">Source</th>
          <th scope="col" className="text-center"><FontAwesomeIcon icon={faStar} /></th>
          <th scope="col">Solutions</th>
        </tr>
      </thead>
      <tbody>
        {problems.map(p =>
          <tr>
            <td className="text-center text-success"><FontAwesomeIcon icon={faCheckCircle} /></td>
            <td className="text-center">{p.point}</td>
            <td><a href={p.url} target="_blank" rel="noopener noreferrer">{p.title}</a></td>
            <td>{p.source}</td>
            <td className="text-center">{p.stars}</td>
            <td>{p.solutions}</td>
          </tr>
        )}
      </tbody>
    </table>
  );
}

const App: React.FC = () => {
  const [problems, setProblems] = useState([]);

  useEffect(() => {
    fetch("/api/v1/problems")
      .then(res => res.json())
      .then(res => setProblems(res));
  }, []);

  return (
    <div className="container">
      <Form />
      <Table problems={problems} />
    </div>
  );
}

export default App;
