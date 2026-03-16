import { useState, useEffect, useMemo } from "react";

const STATION = {
  length: 32000,
  diameter: 8000,
  radius: 4000,
  omega: 0.04964,
  rpm: 0.474,
  period: 126.6,
  rimVelocity: 197.6,
  population: 1000000,
  habitableArea: 500,
  atmosphere: { o2: 40, n2: 60, pressure: 0.5 },
};

const MODULES = [
  { id: 1, name: "Structure", icon: "🏗️", status: "active", health: 98, color: "#3b82f6", desc: "Hull integrity, materials, framework" },
  { id: 2, name: "Rotation", icon: "🔄", status: "active", health: 100, color: "#8b5cf6", desc: "Spin dynamics, gravity generation" },
  { id: 3, name: "ECLSS", icon: "🌬️", status: "warning", health: 87, color: "#06b6d4", desc: "Life support, atmosphere, water" },
  { id: 4, name: "Agriculture", icon: "🌾", status: "active", health: 95, color: "#22c55e", desc: "Food production, hydroponics" },
  { id: 5, name: "Power", icon: "⚡", status: "active", health: 99, color: "#eab308", desc: "Solar arrays, nuclear backup" },
  { id: 6, name: "Shielding", icon: "🛡️", status: "active", health: 96, color: "#ef4444", desc: "Radiation protection systems" },
  { id: 7, name: "Habitat", icon: "🏠", status: "active", health: 94, color: "#f97316", desc: "Residential, recreation zones" },
  { id: 8, name: "Transport", icon: "🚄", status: "active", health: 92, color: "#14b8a6", desc: "Internal transit, elevators" },
  { id: 9, name: "Comms", icon: "📡", status: "active", health: 100, color: "#a855f7", desc: "Communications, navigation" },
  { id: 10, name: "Industrial", icon: "🏭", status: "active", health: 91, color: "#78716c", desc: "Manufacturing, recycling" },
  { id: 11, name: "Docking", icon: "🚀", status: "active", health: 97, color: "#e11d48", desc: "External ports, spacecraft" },
  { id: 12, name: "Command", icon: "🖥️", status: "active", health: 100, color: "#6366f1", desc: "Station management, AI" },
];

const GRAVITY_ZONES = [
  { r: 4000, g: 1.0, zone: "Residential Rim", color: "#3b82f6" },
  { r: 3500, g: 0.875, zone: "Parks & Recreation", color: "#22c55e" },
  { r: 3000, g: 0.75, zone: "Light Commercial", color: "#06b6d4" },
  { r: 2000, g: 0.5, zone: "Transit Level", color: "#eab308" },
  { r: 1000, g: 0.25, zone: "Manufacturing", color: "#f97316" },
  { r: 500, g: 0.125, zone: "Assembly", color: "#a855f7" },
  { r: 0, g: 0.0, zone: "Zero-G Axis", color: "#ef4444" },
];

function CylinderVisualization({ rotation }) {
  const w = 600, h = 300;
  const cx = w / 2, cy = h / 2;
  const cylLen = 400, cylR = 80;

  const strips = 6;
  const lines = [];
  for (let i = 0; i < strips; i++) {
    const angle = (i / strips) * Math.PI * 2 + rotation;
    const y = cy + Math.sin(angle) * cylR;
    const isWindow = i % 2 === 0;
    lines.push(
      <line
        key={`strip-${i}`}
        x1={cx - cylLen / 2}
        y1={y}
        x2={cx + cylLen / 2}
        y2={y}
        stroke={isWindow ? "rgba(135,206,250,0.4)" : "rgba(34,197,94,0.6)"}
        strokeWidth={isWindow ? 1 : 2}
        strokeDasharray={isWindow ? "4 4" : "none"}
      />
    );
  }

  const ringCount = 8;
  const rings = [];
  for (let i = 0; i <= ringCount; i++) {
    const x = cx - cylLen / 2 + (i / ringCount) * cylLen;
    rings.push(
      <ellipse
        key={`ring-${i}`}
        cx={x}
        cy={cy}
        rx={6}
        ry={cylR}
        fill="none"
        stroke="rgba(148,163,184,0.3)"
        strokeWidth={1}
      />
    );
  }

  return (
    <svg viewBox={`0 0 ${w} ${h}`} style={{ width: "100%", maxHeight: 280 }}>
      <defs>
        <linearGradient id="cylGrad" x1="0" y1="0" x2="0" y2="1">
          <stop offset="0%" stopColor="rgba(148,163,184,0.15)" />
          <stop offset="50%" stopColor="rgba(148,163,184,0.05)" />
          <stop offset="100%" stopColor="rgba(148,163,184,0.15)" />
        </linearGradient>
        <radialGradient id="endCap">
          <stop offset="0%" stopColor="rgba(250,204,21,0.6)" />
          <stop offset="70%" stopColor="rgba(250,204,21,0.1)" />
          <stop offset="100%" stopColor="transparent" />
        </radialGradient>
        <filter id="glow">
          <feGaussianBlur stdDeviation="3" result="blur" />
          <feMerge>
            <feMergeNode in="blur" />
            <feMergeNode in="SourceGraphic" />
          </feMerge>
        </filter>
      </defs>

      <rect
        x={cx - cylLen / 2}
        y={cy - cylR}
        width={cylLen}
        height={cylR * 2}
        fill="url(#cylGrad)"
        stroke="rgba(148,163,184,0.4)"
        strokeWidth={1.5}
        rx={4}
      />

      {rings}
      {lines}

      <ellipse cx={cx + cylLen / 2} cy={cy} rx={8} ry={cylR} fill="url(#endCap)" />
      <ellipse cx={cx + cylLen / 2} cy={cy} rx={8} ry={cylR} fill="none" stroke="rgba(250,204,21,0.5)" strokeWidth={1} />

      <ellipse cx={cx - cylLen / 2} cy={cy} rx={8} ry={cylR} fill="rgba(30,41,59,0.8)" stroke="rgba(148,163,184,0.4)" strokeWidth={1} />

      <line x1={cx} y1={cy - cylR - 20} x2={cx} y2={cy + cylR + 20} stroke="rgba(148,163,184,0.2)" strokeWidth={0.5} strokeDasharray="3 3" />
      <text x={cx + 5} y={cy - cylR - 25} fill="rgba(148,163,184,0.6)" fontSize={9} fontFamily="monospace">Central Axis</text>

      {[[-1, "Docking"], [1, "Light Source"]].map(([dir, label]) => (
        <text
          key={label}
          x={cx + (dir * (cylLen / 2 + 30))}
          y={cy + cylR + 20}
          fill="rgba(148,163,184,0.5)"
          fontSize={8}
          fontFamily="monospace"
          textAnchor="middle"
        >
          {label}
        </text>
      ))}

      <g>
        <text x={cx - cylLen / 2} y={cy - cylR - 30} fill="rgba(148,163,184,0.4)" fontSize={8} fontFamily="monospace" textAnchor="middle">
          ⌀ 8 km
        </text>
        <line x1={cx - cylLen / 2} y1={cy - cylR - 15} x2={cx - cylLen / 2} y2={cy - cylR - 5} stroke="rgba(148,163,184,0.3)" strokeWidth={0.5} />
      </g>

      <text x={cx} y={h - 10} fill="rgba(148,163,184,0.4)" fontSize={8} fontFamily="monospace" textAnchor="middle">
        32 km length — Not to scale
      </text>

      <g transform={`translate(${cx}, ${cy})`}>
        {[0, 1, 2, 3].map(i => {
          const a = rotation * 3 + (i * Math.PI / 2);
          const r2 = cylR + 15;
          return (
            <circle
              key={`sat-${i}`}
              cx={Math.cos(a) * 0 + (i % 2 === 0 ? -cylLen/2 - 25 : cylLen/2 + 25)}
              cy={Math.sin(a + i) * r2 * 0.3}
              r={2}
              fill="rgba(250,204,21,0.8)"
              filter="url(#glow)"
            />
          );
        })}
      </g>
    </svg>
  );
}

function CrossSectionView({ rotation }) {
  const size = 260;
  const cx = size / 2, cy = size / 2;
  const maxR = 105;

  return (
    <svg viewBox={`0 0 ${size} ${size}`} style={{ width: "100%", maxHeight: 250 }}>
      <defs>
        <radialGradient id="csGrad">
          <stop offset="0%" stopColor="rgba(135,206,250,0.05)" />
          <stop offset="80%" stopColor="rgba(34,197,94,0.1)" />
          <stop offset="100%" stopColor="rgba(148,163,184,0.2)" />
        </radialGradient>
      </defs>

      <circle cx={cx} cy={cy} r={maxR} fill="url(#csGrad)" stroke="rgba(148,163,184,0.4)" strokeWidth={1.5} />

      {GRAVITY_ZONES.slice(0, -1).map((zone, i) => {
        const r = (zone.r / 4000) * maxR;
        return (
          <circle key={i} cx={cx} cy={cy} r={r} fill="none" stroke={zone.color} strokeWidth={0.5} strokeDasharray="2 3" opacity={0.5} />
        );
      })}

      {[0, 1, 2, 3, 4, 5].map(i => {
        const angle = (i / 6) * Math.PI * 2 + rotation;
        const isWindow = i % 2 === 0;
        const innerR = maxR * 0.92;
        const outerR = maxR;
        const a1 = angle - Math.PI / 6.5;
        const a2 = angle + Math.PI / 6.5;
        const path = `M ${cx + Math.cos(a1) * innerR} ${cy + Math.sin(a1) * innerR}
          A ${innerR} ${innerR} 0 0 1 ${cx + Math.cos(a2) * innerR} ${cy + Math.sin(a2) * innerR}
          L ${cx + Math.cos(a2) * outerR} ${cy + Math.sin(a2) * outerR}
          A ${outerR} ${outerR} 0 0 0 ${cx + Math.cos(a1) * outerR} ${cy + Math.sin(a1) * outerR} Z`;
        return (
          <path
            key={`strip-cs-${i}`}
            d={path}
            fill={isWindow ? "rgba(135,206,250,0.15)" : "rgba(34,197,94,0.25)"}
            stroke={isWindow ? "rgba(135,206,250,0.3)" : "rgba(34,197,94,0.4)"}
            strokeWidth={0.5}
          />
        );
      })}

      <circle cx={cx} cy={cy} r={3} fill="rgba(250,204,21,0.8)" />
      <circle cx={cx} cy={cy} r={8} fill="none" stroke="rgba(250,204,21,0.3)" strokeWidth={0.5} />

      <text x={cx} y={20} fill="rgba(148,163,184,0.6)" fontSize={8} fontFamily="monospace" textAnchor="middle">Cross-Section View</text>

      {GRAVITY_ZONES.filter(z => z.r > 0 && z.r <= 4000).slice(0, 4).map((zone, i) => {
        const r = (zone.r / 4000) * maxR;
        return (
          <text key={`label-${i}`} x={cx + r + 5} y={cy - 3} fill={zone.color} fontSize={6} fontFamily="monospace" opacity={0.7}>
            {zone.g}g
          </text>
        );
      })}
    </svg>
  );
}

function ModuleCard({ module, selected, onClick }) {
  const statusColor = module.status === "active" ? "#22c55e" : module.status === "warning" ? "#eab308" : "#ef4444";
  return (
    <div
      onClick={() => onClick(module.id)}
      style={{
        background: selected ? "rgba(59,130,246,0.15)" : "rgba(15,23,42,0.6)",
        border: `1px solid ${selected ? module.color : "rgba(148,163,184,0.15)"}`,
        borderRadius: 8,
        padding: "10px 12px",
        cursor: "pointer",
        transition: "all 0.2s",
        display: "flex",
        alignItems: "center",
        gap: 10,
      }}
    >
      <span style={{ fontSize: 20 }}>{module.icon}</span>
      <div style={{ flex: 1, minWidth: 0 }}>
        <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
          <span style={{ color: "#e2e8f0", fontSize: 13, fontWeight: 600 }}>{module.name}</span>
          <span style={{ width: 8, height: 8, borderRadius: "50%", background: statusColor, flexShrink: 0 }} />
        </div>
        <div style={{ color: "rgba(148,163,184,0.7)", fontSize: 10, marginTop: 2, whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
          {module.desc}
        </div>
        <div style={{ marginTop: 4, height: 3, background: "rgba(148,163,184,0.1)", borderRadius: 2, overflow: "hidden" }}>
          <div style={{ height: "100%", width: `${module.health}%`, background: module.color, borderRadius: 2, transition: "width 0.5s" }} />
        </div>
      </div>
    </div>
  );
}

function GravityCalculator() {
  const [radius, setRadius] = useState(4000);
  const gravity = (radius / STATION.radius) * 9.81;
  const zone = GRAVITY_ZONES.find(z => radius >= z.r) || GRAVITY_ZONES[GRAVITY_ZONES.length - 1];
  const coriolisWalk = 2 * STATION.omega * 1.5;

  return (
    <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16 }}>
      <h3 style={{ color: "#e2e8f0", fontSize: 14, margin: "0 0 12px", fontFamily: "monospace" }}>Gravity Calculator</h3>
      <div style={{ marginBottom: 8 }}>
        <label style={{ color: "rgba(148,163,184,0.7)", fontSize: 11, fontFamily: "monospace" }}>
          Radius: {radius}m
        </label>
        <input
          type="range"
          min={0}
          max={4000}
          step={50}
          value={radius}
          onChange={e => setRadius(Number(e.target.value))}
          style={{ width: "100%", accentColor: zone.color, marginTop: 4 }}
        />
      </div>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 8, fontSize: 11, fontFamily: "monospace" }}>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 8, borderRadius: 6 }}>
          <div style={{ color: "rgba(148,163,184,0.6)" }}>Gravity</div>
          <div style={{ color: zone.color, fontSize: 18, fontWeight: 700 }}>{(gravity / 9.81).toFixed(3)}g</div>
          <div style={{ color: "rgba(148,163,184,0.5)" }}>{gravity.toFixed(2)} m/s²</div>
        </div>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 8, borderRadius: 6 }}>
          <div style={{ color: "rgba(148,163,184,0.6)" }}>Zone</div>
          <div style={{ color: zone.color, fontSize: 13, fontWeight: 600, marginTop: 2 }}>{zone.zone}</div>
        </div>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 8, borderRadius: 6 }}>
          <div style={{ color: "rgba(148,163,184,0.6)" }}>Coriolis (walking)</div>
          <div style={{ color: "#eab308", fontSize: 14, fontWeight: 600 }}>{(coriolisWalk * (radius / 4000)).toFixed(3)} m/s²</div>
        </div>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 8, borderRadius: 6 }}>
          <div style={{ color: "rgba(148,163,184,0.6)" }}>Rim velocity</div>
          <div style={{ color: "#8b5cf6", fontSize: 14, fontWeight: 600 }}>{(STATION.omega * radius).toFixed(1)} m/s</div>
        </div>
      </div>
    </div>
  );
}

function StationMetrics({ time }) {
  const dayProgress = ((time % 86400) / 86400) * 100;
  const isDaytime = dayProgress > 20 && dayProgress < 80;

  const metrics = [
    { label: "Population", value: "1,042,387", unit: "", color: "#3b82f6" },
    { label: "Rotation", value: STATION.rpm.toFixed(3), unit: "RPM", color: "#8b5cf6" },
    { label: "Rim Gravity", value: "0.98", unit: "g", color: "#22c55e" },
    { label: "O₂ Level", value: "40.2", unit: "%", color: "#06b6d4" },
    { label: "Pressure", value: "50.8", unit: "kPa", color: "#eab308" },
    { label: "Temperature", value: "22.4", unit: "°C", color: "#f97316" },
    { label: "Humidity", value: "52", unit: "%", color: "#14b8a6" },
    { label: "Cycle", value: isDaytime ? "DAY" : "NIGHT", unit: `${dayProgress.toFixed(0)}%`, color: isDaytime ? "#fbbf24" : "#6366f1" },
  ];

  return (
    <div style={{ display: "grid", gridTemplateColumns: "repeat(4, 1fr)", gap: 8 }}>
      {metrics.map((m, i) => (
        <div key={i} style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.1)", borderRadius: 8, padding: "8px 10px", textAlign: "center" }}>
          <div style={{ color: "rgba(148,163,184,0.5)", fontSize: 9, fontFamily: "monospace", textTransform: "uppercase", letterSpacing: 1 }}>{m.label}</div>
          <div style={{ color: m.color, fontSize: 18, fontWeight: 700, fontFamily: "monospace", marginTop: 2 }}>{m.value}</div>
          <div style={{ color: "rgba(148,163,184,0.4)", fontSize: 9, fontFamily: "monospace" }}>{m.unit}</div>
        </div>
      ))}
    </div>
  );
}

function AtmosphereGauge() {
  const gases = [
    { name: "N₂", pct: 60, color: "#3b82f6" },
    { name: "O₂", pct: 40, color: "#22c55e" },
  ];
  let offset = 0;

  return (
    <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16 }}>
      <h3 style={{ color: "#e2e8f0", fontSize: 14, margin: "0 0 12px", fontFamily: "monospace" }}>Atmosphere Composition</h3>
      <div style={{ display: "flex", height: 24, borderRadius: 6, overflow: "hidden", marginBottom: 8 }}>
        {gases.map((g, i) => {
          const style = { width: `${g.pct}%`, background: g.color, display: "flex", alignItems: "center", justifyContent: "center" };
          return (
            <div key={i} style={style}>
              <span style={{ color: "#fff", fontSize: 10, fontWeight: 700, fontFamily: "monospace" }}>{g.name} {g.pct}%</span>
            </div>
          );
        })}
      </div>
      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 6, fontSize: 10, fontFamily: "monospace" }}>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 6, borderRadius: 4, textAlign: "center" }}>
          <div style={{ color: "rgba(148,163,184,0.5)" }}>Pressure</div>
          <div style={{ color: "#eab308", fontWeight: 700 }}>0.5 atm</div>
        </div>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 6, borderRadius: 4, textAlign: "center" }}>
          <div style={{ color: "rgba(148,163,184,0.5)" }}>CO₂</div>
          <div style={{ color: "#22c55e", fontWeight: 700 }}>412 ppm</div>
        </div>
        <div style={{ background: "rgba(0,0,0,0.3)", padding: 6, borderRadius: 4, textAlign: "center" }}>
          <div style={{ color: "rgba(148,163,184,0.5)" }}>Temp</div>
          <div style={{ color: "#f97316", fontWeight: 700 }}>22.4°C</div>
        </div>
      </div>
    </div>
  );
}

export default function CooperStationDashboard() {
  const [time, setTime] = useState(0);
  const [selectedModule, setSelectedModule] = useState(null);
  const [view, setView] = useState("overview");

  useEffect(() => {
    const interval = setInterval(() => setTime(t => t + 1), 50);
    return () => clearInterval(interval);
  }, []);

  const rotation = time * 0.02;

  return (
    <div style={{
      minHeight: "100vh",
      background: "linear-gradient(180deg, #0a0e1a 0%, #0f172a 50%, #0a0e1a 100%)",
      color: "#e2e8f0",
      fontFamily: "'Inter', -apple-system, sans-serif",
      padding: 20,
    }}>
      {/* Header */}
      <div style={{ textAlign: "center", marginBottom: 24 }}>
        <div style={{ fontSize: 10, fontFamily: "monospace", color: "rgba(148,163,184,0.4)", letterSpacing: 4, textTransform: "uppercase", marginBottom: 4 }}>
          Open Source Space Habitat Engineering
        </div>
        <h1 style={{ fontSize: 32, fontWeight: 800, margin: 0, background: "linear-gradient(135deg, #e2e8f0, #94a3b8)", WebkitBackgroundClip: "text", WebkitTextFillColor: "transparent" }}>
          COOPER STATION
        </h1>
        <div style={{ fontSize: 11, fontFamily: "monospace", color: "rgba(148,163,184,0.5)", marginTop: 4 }}>
          O'Neill Cylinder — Island III Variant — Saturn Orbit
        </div>
      </div>

      {/* Navigation */}
      <div style={{ display: "flex", justifyContent: "center", gap: 8, marginBottom: 20 }}>
        {["overview", "modules", "physics"].map(v => (
          <button
            key={v}
            onClick={() => setView(v)}
            style={{
              background: view === v ? "rgba(59,130,246,0.2)" : "rgba(15,23,42,0.6)",
              border: `1px solid ${view === v ? "rgba(59,130,246,0.5)" : "rgba(148,163,184,0.15)"}`,
              borderRadius: 6,
              padding: "6px 16px",
              color: view === v ? "#60a5fa" : "rgba(148,163,184,0.6)",
              fontSize: 11,
              fontFamily: "monospace",
              cursor: "pointer",
              textTransform: "uppercase",
              letterSpacing: 1,
            }}
          >
            {v}
          </button>
        ))}
      </div>

      {/* Station Metrics Bar */}
      <StationMetrics time={time} />

      {view === "overview" && (
        <div style={{ marginTop: 20 }}>
          {/* Cylinder Visualization */}
          <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16, marginBottom: 16 }}>
            <div style={{ display: "flex", gap: 16, alignItems: "center", flexWrap: "wrap" }}>
              <div style={{ flex: 2, minWidth: 300 }}>
                <CylinderVisualization rotation={rotation} />
              </div>
              <div style={{ flex: 1, minWidth: 200 }}>
                <CrossSectionView rotation={rotation} />
              </div>
            </div>
          </div>

          {/* Quick Specs */}
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(200px, 1fr))", gap: 12 }}>
            <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16 }}>
              <h3 style={{ color: "#e2e8f0", fontSize: 14, margin: "0 0 12px", fontFamily: "monospace" }}>Station Parameters</h3>
              {[
                ["Length", "32 km"],
                ["Diameter", "8 km"],
                ["Rotation", `${STATION.rpm} RPM`],
                ["Period", `${STATION.period}s`],
                ["Rim Velocity", `${STATION.rimVelocity} m/s`],
                ["Habitable Area", `${STATION.habitableArea} km²`],
                ["Configuration", "Dual counter-rotating"],
              ].map(([k, v], i) => (
                <div key={i} style={{ display: "flex", justifyContent: "space-between", padding: "4px 0", borderBottom: "1px solid rgba(148,163,184,0.05)", fontSize: 11, fontFamily: "monospace" }}>
                  <span style={{ color: "rgba(148,163,184,0.6)" }}>{k}</span>
                  <span style={{ color: "#e2e8f0", fontWeight: 600 }}>{v}</span>
                </div>
              ))}
            </div>
            <AtmosphereGauge />
            <GravityCalculator />
          </div>
        </div>
      )}

      {view === "modules" && (
        <div style={{ marginTop: 20 }}>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(280px, 1fr))", gap: 10 }}>
            {MODULES.map(m => (
              <ModuleCard key={m.id} module={m} selected={selectedModule === m.id} onClick={setSelectedModule} />
            ))}
          </div>
          {selectedModule && (
            <div style={{ marginTop: 16, background: "rgba(15,23,42,0.6)", border: `1px solid ${MODULES[selectedModule - 1].color}40`, borderRadius: 10, padding: 20 }}>
              <div style={{ display: "flex", alignItems: "center", gap: 12, marginBottom: 12 }}>
                <span style={{ fontSize: 28 }}>{MODULES[selectedModule - 1].icon}</span>
                <div>
                  <h2 style={{ margin: 0, fontSize: 18, color: "#e2e8f0" }}>Module {String(selectedModule).padStart(2, "0")}: {MODULES[selectedModule - 1].name}</h2>
                  <p style={{ margin: "4px 0 0", color: "rgba(148,163,184,0.6)", fontSize: 12 }}>{MODULES[selectedModule - 1].desc}</p>
                </div>
              </div>
              <div style={{ display: "grid", gridTemplateColumns: "repeat(3, 1fr)", gap: 8 }}>
                <div style={{ background: "rgba(0,0,0,0.3)", padding: 10, borderRadius: 6, textAlign: "center", fontFamily: "monospace" }}>
                  <div style={{ color: "rgba(148,163,184,0.5)", fontSize: 9 }}>HEALTH</div>
                  <div style={{ color: MODULES[selectedModule - 1].color, fontSize: 24, fontWeight: 700 }}>{MODULES[selectedModule - 1].health}%</div>
                </div>
                <div style={{ background: "rgba(0,0,0,0.3)", padding: 10, borderRadius: 6, textAlign: "center", fontFamily: "monospace" }}>
                  <div style={{ color: "rgba(148,163,184,0.5)", fontSize: 9 }}>STATUS</div>
                  <div style={{ color: MODULES[selectedModule - 1].status === "active" ? "#22c55e" : "#eab308", fontSize: 14, fontWeight: 700, textTransform: "uppercase", marginTop: 4 }}>
                    {MODULES[selectedModule - 1].status}
                  </div>
                </div>
                <div style={{ background: "rgba(0,0,0,0.3)", padding: 10, borderRadius: 6, textAlign: "center", fontFamily: "monospace" }}>
                  <div style={{ color: "rgba(148,163,184,0.5)", fontSize: 9 }}>CONTRIBUTORS</div>
                  <div style={{ color: "#e2e8f0", fontSize: 24, fontWeight: 700 }}>—</div>
                  <div style={{ color: "rgba(148,163,184,0.4)", fontSize: 9 }}>Open for contribution</div>
                </div>
              </div>
            </div>
          )}
        </div>
      )}

      {view === "physics" && (
        <div style={{ marginTop: 20, display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
          <GravityCalculator />
          <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16 }}>
            <h3 style={{ color: "#e2e8f0", fontSize: 14, margin: "0 0 12px", fontFamily: "monospace" }}>Gravity Zones</h3>
            {GRAVITY_ZONES.map((z, i) => (
              <div key={i} style={{ display: "flex", alignItems: "center", gap: 8, padding: "6px 0", borderBottom: "1px solid rgba(148,163,184,0.05)" }}>
                <div style={{ width: 8, height: 8, borderRadius: "50%", background: z.color, flexShrink: 0 }} />
                <span style={{ color: z.color, fontSize: 12, fontFamily: "monospace", fontWeight: 700, width: 40 }}>{z.g}g</span>
                <span style={{ color: "rgba(148,163,184,0.6)", fontSize: 11, fontFamily: "monospace", flex: 1 }}>{z.zone}</span>
                <span style={{ color: "rgba(148,163,184,0.4)", fontSize: 10, fontFamily: "monospace" }}>r={z.r}m</span>
              </div>
            ))}
          </div>
          <div style={{ background: "rgba(15,23,42,0.6)", border: "1px solid rgba(148,163,184,0.15)", borderRadius: 10, padding: 16 }}>
            <h3 style={{ color: "#e2e8f0", fontSize: 14, margin: "0 0 12px", fontFamily: "monospace" }}>Coriolis Effects</h3>
            <div style={{ fontSize: 11, fontFamily: "monospace", color: "rgba(148,163,184,0.7)", lineHeight: 1.8 }}>
              <div style={{ marginBottom: 8 }}>
                <span style={{ color: "#eab308" }}>Formula:</span> F = -2m(w x v)
              </div>
              {[
                ["Walking (1.5 m/s)", 1.5],
                ["Running (4 m/s)", 4],
                ["Elevator (10 m/s)", 10],
                ["Thrown ball (15 m/s)", 15],
              ].map(([label, v], i) => {
                const accel = 2 * STATION.omega * v;
                return (
                  <div key={i} style={{ display: "flex", justifyContent: "space-between", padding: "3px 0" }}>
                    <span>{label}</span>
                    <span style={{ color: accel > 0.5 ? "#ef4444" : accel > 0.2 ? "#eab308" : "#22c55e" }}>
                      {accel.toFixed(3)} m/s²
                    </span>
                  </div>
                );
              })}
            </div>
          </div>
          <AtmosphereGauge />
        </div>
      )}

      {/* Footer */}
      <div style={{ textAlign: "center", marginTop: 32, padding: "16px 0", borderTop: "1px solid rgba(148,163,184,0.1)" }}>
        <div style={{ color: "rgba(148,163,184,0.3)", fontSize: 10, fontFamily: "monospace", fontStyle: "italic" }}>
          "Mankind was born on Earth. It was never meant to die here."
        </div>
        <div style={{ color: "rgba(148,163,184,0.2)", fontSize: 9, fontFamily: "monospace", marginTop: 4 }}>
          Cooper Station Project — Open Source Space Habitat Engineering
        </div>
      </div>
    </div>
  );
}
