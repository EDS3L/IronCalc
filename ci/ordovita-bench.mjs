// Compares two wasm builds of the engine (usually the upstream tag and this
// branch) on lookup-heavy sheets: recalculation time, time of one edit, and
// that every formula gives the same text in both.
//
// node ci/ordovita-bench.mjs <dir of the "before" nodejs build> <dir of the "after" nodejs build>
import { createRequire } from 'node:module';
import path from 'node:path';

const require = createRequire(import.meta.url);
const [beforeDir, afterDir] = process.argv.slice(2);
const engines = {
  before: require(path.resolve(beforeDir, 'wasm.js')),
  after: require(path.resolve(afterDir, 'wasm.js')),
};

const ROWS = 10000;
const variants = {
  'arithmetic =C*E (10k)': (r) => [[6, `=C${r}*E${r}`]],
  'VLOOKUP exact into 1k rows (10k)': (r) => [[8, `=VLOOKUP(MOD(A${r},1000)+1,Lookup!$A$1:$B$1000,2,FALSE)`]],
  'VLOOKUP exact text keys (10k)': (r) => [[8, `=VLOOKUP("k"&(MOD(A${r},1000)+1),Lookup!$C$1:$D$1000,2,FALSE)`]],
  'VLOOKUP sorted (10k)': (r) => [[8, `=VLOOKUP(MOD(A${r},1000)+1,Lookup!$A$1:$B$1000,2,TRUE)`]],
  'INDEX/MATCH exact (10k)': (r) => [[8, `=INDEX(Lookup!$B$1:$B$1000,MATCH(MOD(A${r},1000)+1,Lookup!$A$1:$A$1000,0))`]],
  'XLOOKUP exact (10k)': (r) => [[8, `=XLOOKUP(MOD(A${r},1000)+1,Lookup!$A$1:$A$1000,Lookup!$B$1:$B$1000)`]],
  'VLOOKUP whole column A:B (1k)': (r) => (r <= 1000 ? [[8, `=VLOOKUP(MOD(A${r},1000)+1,Lookup!A:B,2,FALSE)`]] : []),
  'MATCH whole column A:A (1k)': (r) => (r <= 1000 ? [[8, `=MATCH(MOD(A${r},1000)+1,Lookup!A:A,0)`]] : []),
  'COUNTIFS every row (1k x 10k)': (r) =>
    r <= 1000 ? [[11, `=COUNTIFS($B$1:$B$${ROWS},B${r},$E$1:$E$${ROWS},">"&E${r})`]] : [],
};
const base = (r) => [[1, r], [2, 'cat' + (r % 20)], [3, ((r * 37) % 1000) + 0.25], [5, (r % 9) + 1]];

function run(engine, fx) {
  const m = new engine.Model('P', 'en', 'UTC', 'en');
  m.newSheet();
  m.renameSheet(1, 'Lookup');
  m.pauseEvaluation();
  for (let r = 1; r <= 1000; r++) {
    m.setUserInput(1, r, 1, String(r));
    m.setUserInput(1, r, 2, String(r * 2.5));
    m.setUserInput(1, r, 3, 'k' + r);
    m.setUserInput(1, r, 4, 'text ' + r);
  }
  const formulas = [];
  for (let r = 1; r <= ROWS; r++) {
    for (const [c, v] of base(r)) m.setUserInput(0, r, c, String(v));
    for (const [c, f] of fx(r)) {
      m.setUserInput(0, r, c, f);
      formulas.push([r, c]);
    }
  }
  m.resumeEvaluation();
  let t0 = performance.now();
  m.evaluate();
  const full = performance.now() - t0;
  t0 = performance.now();
  m.setUserInput(0, 1, 3, '999');
  m.evaluate();
  const edit = performance.now() - t0;
  const values = formulas.map(([r, c]) => m.getFormattedCellValue(0, r, c));
  m.free();
  return { full, edit, values };
}

const lines = [
  '| scenario | before: recalc ms | after: recalc ms | before: edit ms | after: edit ms | results |',
  '|---|---|---|---|---|---|',
];
let mismatches = 0;
for (const [name, fx] of Object.entries(variants)) {
  const a = run(engines.before, fx);
  const b = run(engines.after, fx);
  let diff = 0;
  let example = '';
  for (let i = 0; i < a.values.length; i++) {
    if (a.values[i] !== b.values[i]) {
      diff++;
      if (!example) example = ` (e.g. ${JSON.stringify(a.values[i])} vs ${JSON.stringify(b.values[i])})`;
    }
  }
  mismatches += diff;
  lines.push(
    `| ${name} | ${a.full.toFixed(1)} | ${b.full.toFixed(1)} | ${a.edit.toFixed(1)} | ${b.edit.toFixed(1)} | ${diff === 0 ? `same (${a.values.length})` : `${diff} different${example}`} |`,
  );
}
console.log(lines.join('\n'));
if (mismatches > 0) {
  console.log(`\n${mismatches} results differ between the builds.`);
  process.exitCode = 1;
}
