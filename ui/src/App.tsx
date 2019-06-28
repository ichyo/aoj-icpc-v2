import React, { useState, useEffect, FormEventHandler, FormEvent } from 'react';
import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheckCircle, faStar, faUserInjured } from '@fortawesome/free-solid-svg-icons'

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

interface TableProps {
  problems: Array<Problem>,
  user: User | null,
}

interface FormData {
  aojUserId: string,
}

interface FormProps {
  onSubmit: (data: FormData) => void,
}

const Form: React.FC<FormProps> = ({ onSubmit }) => {
  const [aojUserId, setAojUserId] = useState("");

  const handleSubmit = (event: FormEvent) => {
    event.preventDefault();
    const data = {
      aojUserId,
    };
    onSubmit(data);
  };

  return (
    <form className="form-inline mb-3 mt-3" onSubmit={handleSubmit}>
      <input
        type="text"
        className="form-control mr-2 col-4 col-md-3"
        placeholder="AOJ ID"
        value={aojUserId}
        onChange={e => setAojUserId(e.target.value)}
      />
      <button type="submit" className="btn btn-primary">Update</button>
    </form>
  );
}

const Table: React.FC<TableProps> = ({ problems, user }) => {
  const solutions = new Set(user ? user.solutions : []);
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

const App: React.FC = () => {
  const [problems, setProblems] = useState([]);
  const [user, setUser] = useState(null);

  useEffect(() => {
    fetch("/api/v1/problems")
      .then(res => res.json())
      .then(res => setProblems(res));
  }, []);

  const handleSubmit = (data: FormData) => {
    if (data.aojUserId) {
      fetch("/api/v1/aoj_users/" + data.aojUserId)
        .then(res => res.json())
        .then(res => setUser(res), err => console.log(err)); // TODO: error handling
    }
  };

  return (
    <div className="container">
      <Form onSubmit={handleSubmit} />
      <Table problems={problems} user={user} />
    </div>
  );
}

export default App;
