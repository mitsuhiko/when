import * as wasm from "when";
import React, { useEffect, useState, useRef } from "react";
import ReactDOM from "react-dom";
import "./style.css";

wasm.set_panic_hook();

const EXAMPLES = [
  "now",
  "2 hours ago in yyz",
  "5pm in yyz -> sfo",
  "5pm in Vienna -> London",
  "4pm on 17.05.2021 in Vienna -> Tokyo",
  "in 4 hours in San Francisco",
  "2pm in 2 days in New Delhi",
  "now in yyz -> sfo -> vie -> lhr",
  "unix 1639067620 in Tokyo",
];

function parseDateExpr(input) {
  return JSON.parse(wasm.parse_expr(input || "now"));
}

function Location({ location: loc }) {
  const match = loc.datetime.match(/^([^T]+)T([^.]+)/);
  return (
    <table>
      <tbody>
        <tr>
          <th>Time</th>
          <td>
            <span className="time">{match[2]}</span> (
            {loc.time_of_day.replace(/_/g, " ")})
          </td>
        </tr>
        <tr>
          <th>Date</th>
          <td>
            <span className="date">{match[1]}</span>
          </td>
        </tr>
        <tr>
          <th>Zone</th>
          <td>
            <span className="zone">{loc.timezone.name}</span> (
            {loc.timezone.abbrev}; {loc.timezone.utc_offset})
          </td>
        </tr>
        {loc.location && (
          <tr>
            <th>Location</th>
            <td>
              <strong>{loc.location.name}</strong>
              {loc.location.admin_code
                ? ` (${loc.location.admin_code}; ${loc.location.country})`
                : loc.location.country
                ? ` (${loc.location.country})`
                : null}
            </td>
          </tr>
        )}
      </tbody>
    </table>
  );
}

function Examples(setExpr) {
  return <div class="examples">
    <h3>Need inspiration? Try some of these</h3>
    <ul>{EXAMPLES.map((x, idx) => {
      return <li key={idx}><a onClick={() => {setExpr(x);}}>{x}</a></li>;
    })}</ul>
  </div>;
}

function App() {
  const url = new URL(window.location);
  const [inc, setInc] = useState(0);
  const [expr, setExpr] = useState(url.searchParams.get("input") || "");
  const inputRef = useRef();
  const rv = parseDateExpr(expr);

  useEffect(() => {
    const url = new URL(window.location);
    url.searchParams.set("input", expr);
    window.history.replaceState({}, "", url);

    if (rv.is_relative) {
      const timer = setTimeout(() => {
        setInc(inc + 1);
      }, 1000);
      return () => clearTimeout(timer);
    }
  }, [inc, rv.is_relative, location.search]);

  function setExprAndFocus(value) {
    setExpr(value);
    if (inputRef.current) {
      inputRef.current.focus();
    }
  }
  
  return (
    <div>
      <header>
        <h1>when?</h1>
        <a href="https://github.com/mitsuhiko/when">huh?</a>
        {" | "}
        <a href="https://lucumr.pocoo.org/">who?</a>
      </header>
      <a onClick={() => setExprAndFocus('')} title="Clear input (ESC)" className="clear">x</a>
      <input
        type="text"
        value={expr}
        ref={inputRef}
        onKeyUp={(evt) => {
          if (evt.key === "Escape") {
            setExprAndFocus("");
          }
        }}
        onChange={(evt) => {
          setExpr(evt.target.value);
        }}
        size="40"
        autoFocus
      />
      {!expr && Examples(setExprAndFocus)}
      {expr && rv.locations && (
        <ul>
          {rv.locations.map((loc, idx) => (
            <li key={idx}>
              <Location location={loc} />
            </li>
          ))}
        </ul>
      )}
      {rv.error && (
        <p className="error">
          <strong>Ugh:</strong>
          {" " + rv.error + " :-("}
        </p>
      )}
    </div>
  );
}

ReactDOM.render(<App />, document.getElementById("root"));
