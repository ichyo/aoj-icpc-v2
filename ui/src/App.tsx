import React, { useState, useEffect } from 'react';
import './App.css';
import './components/ProblemTable';
import PointSummury from './components/PointSummary';
import ProblemTable from './components/ProblemTable';
import ProblemFilter from './ProblemFilter';
import SearchForm, { FormData } from './components/SearchForm';
import { Problem, User } from './model';


const App: React.FC = () => {
  const [problems, setProblems] = useState([] as Problem[]);
  const [solutions, setSolutions] = useState(new Map());
  const [user, setUser] = useState(null as User | null);
  const [problemFilter, setProblemFilter] = useState(ProblemFilter.default());

  useEffect(() => {
    const apiProblems = fetch("/api/v1/problems")
      .then(res => res.json());

    const apiSolutions = fetch("/api/v1/problems/solutions")
      .then(res => res.json());

    Promise.all([apiProblems, apiSolutions])
      .then(([problems, solutions]) => {
        problems.sort((a: Problem, b: Problem) => {
          if (a.point != b.point) {
            return a.point - b.point
          }
          return (solutions[b.id.toString()] || 0)
            - (solutions[a.id.toString()] || 0);
        })
        setProblems(problems);
        setSolutions(new Map(Object.entries(solutions)));
      });
  }, []);

  const handleSubmit = (data: FormData) => {
    const currentUser = user ? user.id : "";
    if (currentUser !== data.aojUserId) {
      setUser(null);
    }
    if (data.aojUserId) {
      fetch("/api/v1/aoj_users/" + data.aojUserId)
        .then(res => res.json())
        .then(res => setUser(res), err => console.log(err)); // TODO: error handling
    }

    const builder = ProblemFilter.builder()
      .setMinimumPoint(data.minimumPoint)
      .setMaximumPoint(data.maximumPoint)
      .setHideAC(data.hideAC)
      .setShowPending(data.showPending)
      .setSinceYear(data.sinceYear)
      .setUntilYear(data.untilYear);

    const filter = builder.build();
    setProblemFilter(filter);
  };

  const handleSort = (comparator: (a: Problem, b: Problem) => number) => {
    const sorted_problems = Array.from(problems.sort(comparator));
    setProblems(sorted_problems);
  }

  const points = Array.from(new Set(problems.map(p => p.point).filter(p => p != 0))).sort((a, b) => a - b);
  const years = Array.from(new Set(problems.map(p => p.year))).sort((a, b) => a - b);
  const filteredProblems = problemFilter.filters(problems, user);

  return (
    <div className="container">
      <SearchForm onSubmit={handleSubmit} points={points} years={years} />
      {user ? <PointSummury problems={filteredProblems} user={user} /> : null}
      <ProblemTable
        problems={filteredProblems}
        user={user}
        solutions={solutions}
        handleSort={handleSort}
      />
    </div>
  );
}

export default App;
