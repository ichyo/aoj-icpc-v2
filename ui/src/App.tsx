import React from 'react';
import './App.css';

interface Problem {
  point: number,
  title: string,
  source: string,
  solutions: number,
  url: string,
  stars: number,
}

const problems = [
  {
    point: 100,
    title: 'ICPC 得点集計ソフトウェア',
    source: '国内予選2007A',
    solutions: 2692,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1147&lang=jp',
    stars: 0,
  },
  {
    point: 100,
    title: 'Hanafuda Shuffle',
    source: '国内予選2004A',
    solutions: 2076,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1129&lang=jp',
    stars: 0,
  },
  {
    point: 150,
    title: 'Red and Black',
    source: '国内予選2004B',
    solutions: 1887,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1130&lang=jp',
    stars: 0,
  },
  {
    point: 250,
    title: 'ポロック予想',
    source: '国内予選2010C',
    solutions: 944,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1167&lang=jp',
    stars: 0,
  },
  {
    point: 450,
    title: 'Circle and Points',
    source: '国内予選2004D',
    solutions: 525,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1132&lang=jp',
    stars: 4,
  },
  {
    point: 800,
    title: '壊れたドア',
    source: '国内予選2011G',
    solutions: 112,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1178&lang=jp',
    stars: 3,
  },
  {
    point: 900,
    title: 'Common Palindromes',
    source: '夏合宿2011:day2C',
    solutions: 57,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2292&lang=jp',
    stars: 0,
  },
  {
    point: 1100,
    title: 'How to Create a Good Game',
    source: '夏合宿2010:day4I',
    solutions: 57,
    url: 'http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=2230&lang=jp',
    stars: 5,
  },
]

interface TableProps {
  problems: Array<Problem>,
}

const Table: React.FC<TableProps> = ({ problems }) => {
  return (
    <table className="table table-sm">
      <thead>
        <tr>
          <th scope="col" className="text-center">Point</th>
          <th scope="col">Title</th>
          <th scope="col">Source</th>
          <th scope="col">Solutions</th>
        </tr>
      </thead>
      <tbody>
        {problems.map(p =>
          <tr>
            <td className="text-center">{p.point}</td>
            <td><a href={p.url}>{p.title}</a></td>
            <td>{p.source}</td>
            <td>{p.solutions}</td>
          </tr>
        )}
      </tbody>
    </table>
  );
}

const App: React.FC = () => {
  return (
    <div className="container">
      <Table problems={problems} />
    </div>
  );
}

export default App;
