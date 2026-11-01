//! Visualisasi & Web UI Dashboard Interaktif untuk Modul 2 (IoT Communication, Edge Processing, & RBAC).
//!
//! Disajikan langsung oleh Axum pada rute `GET /` untuk memenuhi komponen
//! **Visualisasi atau UI (25%)** dan **Demonstrasi End-to-End (20%)** pada rubrik penilaian.

pub const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Smart Soil IoT — Modul 2: Edge Gateway, Offline Buffer & RBAC</title>
  <style>
    :root {
      --bg: #0f172a;
      --card: #1e293b;
      --border: #334155;
      --text: #f8fafc;
      --muted: #94a3b8;
      --emerald: #10b981;
      --amber: #f59e0b;
      --rose: #f43f5e;
      --sky: #38bdf8;
      --indigo: #6366f1;
    }
    * { box-sizing: border-box; margin: 0; padding: 0; font-family: 'Segoe UI', system-ui, sans-serif; }
    body { background: var(--bg); color: var(--text); padding: 20px; line-height: 1.5; }
    .container { max-width: 1320px; margin: 0 auto; }
    header {
      background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
      border: 1px solid var(--border);
      border-radius: 14px;
      padding: 20px 24px;
      margin-bottom: 18px;
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 16px;
    }
    header h1 { font-size: 1.35rem; color: #fff; }
    header p { color: var(--muted); font-size: 0.9rem; margin-top: 4px; }
    .badge-row { display: flex; gap: 10px; flex-wrap: wrap; align-items: center; }
    .badge {
      padding: 6px 12px;
      border-radius: 999px;
      font-size: 0.8rem;
      font-weight: 600;
      border: 1px solid var(--border);
      background: #0f172a;
    }
    .badge.online { background: rgba(16, 185, 129, 0.15); color: #34d399; border-color: #059669; }
    .badge.offline { background: rgba(244, 63, 94, 0.2); color: #fb7185; border-color: #e11d48; }
    .kpi-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(210px, 1fr));
      gap: 14px;
      margin-bottom: 18px;
    }
    .kpi {
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 12px;
      padding: 14px 18px;
    }
    .kpi .label { font-size: 0.8rem; color: var(--muted); text-transform: uppercase; letter-spacing: 0.04em; }
    .kpi .val { font-size: 1.55rem; font-weight: 700; margin-top: 4px; }
    .kpi .sub { font-size: 0.78rem; color: var(--muted); margin-top: 2px; }
    .main-grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 18px;
      margin-bottom: 18px;
    }
    @media (max-width: 980px) { .main-grid { grid-template-columns: 1fr; } }
    .card {
      background: var(--card);
      border: 1px solid var(--border);
      border-radius: 12px;
      padding: 18px 20px;
    }
    .card h2 {
      font-size: 1.05rem;
      margin-bottom: 6px;
      display: flex;
      align-items: center;
      justify-content: space-between;
      color: var(--sky);
    }
    .card .desc { font-size: 0.82rem; color: var(--muted); margin-bottom: 14px; }
    .form-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 10px;
      margin-bottom: 12px;
    }
    .form-group { display: flex; flex-direction: column; gap: 4px; }
    .form-group label { font-size: 0.78rem; color: var(--muted); font-weight: 600; }
    input, select {
      background: #0f172a;
      border: 1px solid var(--border);
      color: #fff;
      padding: 8px 10px;
      border-radius: 8px;
      font-size: 0.88rem;
    }
    .btn-row { display: flex; gap: 8px; flex-wrap: wrap; margin-top: 8px; }
    button {
      cursor: pointer;
      border: none;
      border-radius: 8px;
      padding: 8px 13px;
      font-size: 0.83rem;
      font-weight: 600;
      transition: opacity 0.15s;
    }
    button:hover { opacity: 0.9; }
    .btn-primary { background: var(--indigo); color: #fff; }
    .btn-success { background: var(--emerald); color: #052e16; }
    .btn-warn { background: var(--amber); color: #451a03; }
    .btn-danger { background: var(--rose); color: #fff; }
    .btn-outline { background: transparent; border: 1px solid var(--border); color: var(--text); }
    .alert-box {
      margin-top: 12px;
      padding: 10px 14px;
      border-radius: 8px;
      font-size: 0.84rem;
      background: #0f172a;
      border-left: 4px solid var(--sky);
      white-space: pre-wrap;
    }
    .alert-box.ok { border-left-color: var(--emerald); color: #6ee7b7; }
    .alert-box.err { border-left-color: var(--rose); color: #fda4af; }
    .alert-box.warn { border-left-color: var(--amber); color: #fde68a; }
    table { width: 100%; border-collapse: collapse; font-size: 0.82rem; margin-top: 10px; }
    th, td { border-bottom: 1px solid var(--border); padding: 8px 6px; text-align: left; }
    th { color: var(--muted); font-weight: 600; }
    .pill {
      display: inline-block;
      padding: 2px 8px;
      border-radius: 999px;
      font-size: 0.73rem;
      font-weight: 700;
    }
    .pill.good { background: rgba(16, 185, 129, 0.2); color: #34d399; }
    .pill.suspect { background: rgba(245, 158, 11, 0.2); color: #fbbf24; }
    .pill.bad { background: rgba(244, 63, 94, 0.25); color: #fb7185; }
    pre {
      background: #090d16;
      border: 1px solid var(--border);
      padding: 12px;
      border-radius: 8px;
      font-family: Consolas, monospace;
      font-size: 0.79rem;
      overflow-x: auto;
      color: #a5f3fc;
      max-height: 280px;
    }
  </style>
</head>
<body>
  <div class="container">
    <header>
      <div>
        <h1>🌱 Smart Soil IoT — Modul 2: IoT Communication, Edge Processing & RBAC</h1>
        <p>Arsitektur Pemrograman Fungsional (Pure Core vs Axum Side-Effect Shell) • Kontrak Integrasi M1 (RAB ESP32/SIM7000E) ➔ M2 ➔ M3 (ReadingSink & PlotMeta)</p>
      </div>
      <div class="badge-row">
        <span id="netBadge" class="badge online">● SIM7000E: ONLINE</span>
        <span class="badge">X-API-Key: TubesPF-demo-key</span>
      </div>
    </header>

    <div class="kpi-grid">
      <div class="kpi">
        <div class="label">Perangkat Terdaftar (M2)</div>
        <div class="val" id="kpiDevices">0</div>
        <div class="sub">ESP32 + SIM7000E + ADS1115</div>
      </div>
      <div class="kpi">
        <div class="label">Antrean Offline Buffer</div>
        <div class="val" id="kpiBuffer" style="color: var(--amber);">0</div>
        <div class="sub">Disimpan saat sinyal putus</div>
      </div>
      <div class="kpi">
        <div class="label">Terkirim ke Sink Modul 3</div>
        <div class="val" id="kpiM3" style="color: var(--emerald);">0</div>
        <div class="sub">Format SoilReading + PlotMeta</div>
      </div>
      <div class="kpi">
        <div class="label">Status Buzzer 5V (RAB #9)</div>
        <div class="val" id="kpiBuzzer">AMAN</div>
        <div class="sub">Bunyi jika sensor rusak/putus</div>
      </div>
    </div>

    <div class="main-grid">
      <!-- PANEL 1: EDGE SENSOR & ANOMALY & SAMPLING INTERVAL -->
      <div class="card">
        <h2>
          <span>1. Akuisisi Edge, Deteksi Anomali & Interval Sampling</span>
          <span class="badge">Pure: anomaly.rs</span>
        </h2>
        <p class="desc">Simulasi pengiriman <code>RawReading</code> dari sensor RAB Modul 1 (DFRobot Capacitive Moisture, DS18B20 Suhu, pH + DMS, DFRobot EC) ke server Axum.</p>

        <div class="btn-row" style="margin-bottom: 12px;">
          <button class="btn-outline" onclick="applyPreset('normal')">🌱 Preset Normal Gambut</button>
          <button class="btn-outline" onclick="applyPreset('suspect')">⚠️ Preset Kering Ekstrem (Suspect)</button>
          <button class="btn-outline" onclick="applyPreset('bad')">🚨 Preset Kabel Putus (Bad + Buzzer)</button>
          <button class="btn-outline" onclick="applyPreset('spam')">⏱️ Uji Interval Belum Tercapai</button>
        </div>

        <div class="form-grid">
          <div class="form-group">
            <label>Perangkat (device_id)</label>
            <select id="inDevice"></select>
          </div>
          <div class="form-group">
            <label>Waktu Detik (timestamp_sec)</label>
            <input type="number" id="inTsSec" value="1000" />
          </div>
          <div class="form-group">
            <label>Moisture (% VWC) [DFRobot Capacitive]</label>
            <input type="number" step="0.1" id="inMoisture" value="68.5" />
          </div>
          <div class="form-group">
            <label>Temperature (°C) [DS18B20 Probe]</label>
            <input type="number" step="0.1" id="inTemp" value="28.4" />
          </div>
          <div class="form-group">
            <label>pH Tanah (0-14) [Sensor pH + DMS]</label>
            <input type="number" step="0.1" id="inPh" value="4.5" />
          </div>
          <div class="form-group">
            <label>EC (mS/cm) [DFRobot TDS/EC]</label>
            <input type="number" step="0.1" id="inEc" value="1.2" />
          </div>
        </div>

        <div class="btn-row">
          <button class="btn-primary" onclick="sendSensorData()">🚀 Kirim Data Sensor (POST /api/sensor/data)</button>
        </div>

        <div id="sensorResult" class="alert-box">Siap menerima pembacaan sensor dari ESP32...</div>
      </div>

      <!-- PANEL 2: OFFLINE BUFFERING & RECONNECT -->
      <div class="card">
        <h2>
          <span>2. Konektivitas SIM7000E, Retry & Offline Buffering</span>
          <span class="badge">Pure: retry.rs & pipeline.rs</span>
        </h2>
        <p class="desc">Jika sinyal seluler di lahan gambut putus, data disimpan secara <em>immutable</em> di <code>Offline Buffer</code> dan dikirim otomatis menggunakan <code>fold</code> saat kembali <em>Online</em>.</p>

        <div class="btn-row">
          <button class="btn-danger" onclick="toggleNetwork(false)">📴 Simulasi Sinyal Putus (Set OFFLINE)</button>
          <button class="btn-success" onclick="toggleNetwork(true)">📶 Reconnect & Upload Buffer ke M3 (Set ONLINE)</button>
        </div>

        <div id="bufferStatus" class="alert-box" style="margin-top:12px;">Status koneksi: ONLINE (Langsung meneruskan ke Modul 3)</div>

        <h3 style="font-size:0.88rem; margin-top:14px; color:var(--muted);">Daftar Antrean Offline Buffer Saat Ini:</h3>
        <table>
          <thead>
            <tr>
              <th>Event ID</th>
              <th>Device ID</th>
              <th>Moisture / Suhu / pH / EC</th>
              <th>Alasan Ditahan</th>
            </tr>
          </thead>
          <tbody id="bufferTableBody">
            <tr><td colspan="4" style="color:var(--muted);">Antrean offline kosong.</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="main-grid">
      <!-- PANEL 3: DEVICE DIRECTORY, PLOTMETA & RBAC -->
      <div class="card">
        <h2>
          <span>3. Device Provisioning, Pemetaan PlotMeta & RBAC</span>
          <span class="badge">Pure: device, user, rbac</span>
        </h2>
        <p class="desc">Mengatur kepemilikan perangkat (<code>Device ➔ User ➔ PlotMeta</code>) dan menguji otorisasi <code>Admin</code>, <code>Operator</code> (Petani), dan <code>Viewer</code> (Peneliti).</p>

        <table>
          <thead>
            <tr>
              <th>Device ID</th>
              <th>Pemilik</th>
              <th>Plot ID & Tanaman (PlotMeta M3)</th>
              <th>Interval</th>
            </tr>
          </thead>
          <tbody id="deviceTableBody"></tbody>
        </table>

        <hr style="border:0; border-top:1px solid var(--border); margin: 14px 0;" />
        <h3 style="font-size:0.88rem; margin-bottom:8px; color:var(--sky);">Uji Hak Akses RBAC & Kontrol Interval Sampling:</h3>
        <div class="form-grid">
          <div class="form-group">
            <label>Pilih User & Role</label>
            <select id="rbacUser">
              <option value="admin_utama|Admin">admin_utama (Role: Admin)</option>
              <option value="petani_budi|Operator">petani_budi (Role: Operator - Pemilik ESP32-GAMBUT-01)</option>
              <option value="petani_siti|Operator">petani_siti (Role: Operator - Pemilik ESP32-GAMBUT-02)</option>
              <option value="peneliti_rina|Viewer">peneliti_rina (Role: Viewer - Hanya Baca)</option>
            </select>
          </div>
          <div class="form-group">
            <label>Target Perangkat</label>
            <select id="rbacDevice"></select>
          </div>
          <div class="form-group">
            <label>Aksi RBAC</label>
            <select id="rbacAction">
              <option value="Kontrol">Kontrol (Ubah Interval Sampling)</option>
              <option value="Baca">Baca (Lihat Data Perangkat)</option>
              <option value="Hapus">Hapus (Hapus Perangkat dari Registry)</option>
            </select>
          </div>
          <div class="form-group">
            <label>Interval Baru (detik, jika Kontrol)</label>
            <input type="number" id="rbacInterval" value="30" />
          </div>
        </div>
        <div class="btn-row">
          <button class="btn-warn" onclick="testRbacAction()">🔐 Jalankan Validasi RBAC & Kepemilikan</button>
          <button class="btn-outline" onclick="showAddDeviceModal()">➕ Registrasi Alat & PlotMeta Baru</button>
        </div>
        <div id="rbacResult" class="alert-box">Pilih User, Role, dan Aksi untuk menguji fungsi murni <code>otorisasi_aksi_perangkat</code>.</div>

        <!-- Form Tambah Alat Baru (tersembunyi default) -->
        <div id="addDeviceBox" style="display:none; margin-top:14px; padding-top:12px; border-top:1px dashed var(--border);">
          <h4 style="font-size:0.85rem; margin-bottom:8px;">Form Provisioning Perangkat & PlotMeta (Kontrak M3):</h4>
          <div class="form-grid">
            <div class="form-group"><label>Device ID</label><input id="newDevId" value="ESP32-GAMBUT-03" /></div>
            <div class="form-group"><label>Owner Username</label><input id="newOwner" value="petani_budi" /></div>
            <div class="form-group"><label>Plot ID</label><input id="newPlotId" value="PLOT-KALTENG-03" /></div>
            <div class="form-group"><label>Nama Petak</label><input id="newPlotName" value="Petak Sawit/Nanas Blok C" /></div>
            <div class="form-group"><label>Tanaman (plant)</label><input id="newPlant" value="nanas" /></div>
            <div class="form-group"><label>Luas Lahan (area_ha)</label><input type="number" step="0.1" id="newArea" value="1.8" /></div>
          </div>
          <button class="btn-success" onclick="registerNewDevice()">Simpan Perangkat & PlotMeta</button>
        </div>
      </div>

      <!-- PANEL 4: INTEGRASI KE MODUL 3 (SOILREADING + PLOTMETA) -->
      <div class="card">
        <h2>
          <span>4. Kontrak Integrasi ke Modul 3 (ReadingSink & PlotMeta)</span>
          <span class="badge">Pure: contracts.rs</span>
        </h2>
        <p class="desc">Data yang lolos dari Modul 2 diserahkan ke <code>ReadingSink</code> milik Modul 3 dalam struktur JSON <code>SoilReading</code> dan <code>PlotMeta</code> sesuai kontrak kelas.</p>

        <table>
          <thead>
            <tr>
              <th>Waktu (ts)</th>
              <th>Device & Plot</th>
              <th>Moisture / Temp / pH / EC</th>
              <th>QualityFlag</th>
              <th>Buzzer</th>
            </tr>
          </thead>
          <tbody id="m3TableBody">
            <tr><td colspan="5" style="color:var(--muted);">Belum ada data di ReadingSink M3. Klik "Kirim Data Sensor" di Panel 1!</td></tr>
          </tbody>
        </table>

        <h3 style="font-size:0.85rem; margin: 12px 0 6px; color:var(--muted);">Payload JSON Terakhir yang Dikirim ke Modul 3 (SoilReading + PlotMeta):</h3>
        <pre id="m3JsonPreview">// Kirim data sensor pada Panel 1 untuk melihat output JSON kontrak Modul 3 di sini</pre>
      </div>
    </div>
  </div>

  <script>
    const API_KEY = "TubesPF-demo-key";
    let eventCounter = 1;

    async function loadState() {
      const res = await fetch('/api/state');
      const state = await res.json();

      // Update badge jaringan
      const netBadge = document.getElementById('netBadge');
      if (state.network_online) {
        netBadge.className = 'badge online';
        netBadge.textContent = '● SIM7000E: ONLINE';
      } else {
        netBadge.className = 'badge offline';
        netBadge.textContent = '● SIM7000E: OFFLINE (Buffering)';
      }

      const devices = state.devices || [];
      document.getElementById('kpiDevices').textContent = devices.length;
      document.getElementById('kpiBuffer').textContent = state.offline_buffer.length;
      document.getElementById('kpiM3').textContent = state.m3_sink_records.length;

      // Update status Buzzer dari record terakhir
      const lastRecord = state.m3_sink_records[state.m3_sink_records.length - 1];
      const kpiBuzzer = document.getElementById('kpiBuzzer');
      if (lastRecord && lastRecord.buzzer_alarm_active) {
        kpiBuzzer.textContent = '🚨 BUNYI!';
        kpiBuzzer.style.color = 'var(--rose)';
      } else {
        kpiBuzzer.textContent = 'AMAN';
        kpiBuzzer.style.color = 'var(--emerald)';
      }

      // Isi dropdown perangkat
      const inDev = document.getElementById('inDevice');
      const rbacDev = document.getElementById('rbacDevice');
      const prevDev = inDev.value;
      inDev.innerHTML = '';
      rbacDev.innerHTML = '';
      devices.forEach(d => {
        inDev.innerHTML += `<option value="${d.id}">${d.id} (Interval: ${d.sampling_interval_sec}s)</option>`;
        rbacDev.innerHTML += `<option value="${d.id}">${d.id}</option>`;
      });
      if (prevDev) inDev.value = prevDev;

      // Tabel Device & PlotMeta
      const devBody = document.getElementById('deviceTableBody');
      devBody.innerHTML = devices.map(d => `
        <tr>
          <td><strong>${d.id}</strong></td>
          <td>${d.owner || '-'}</td>
          <td>${d.plot ? `${d.plot.plot_id} (${d.plot.plant}, ${d.plot.area_ha} ha)` : '-'}</td>
          <td>${d.sampling_interval_sec}s</td>
        </tr>
      `).join('');

      // Tabel Offline Buffer
      const bufBody = document.getElementById('bufferTableBody');
      if (state.offline_buffer.length === 0) {
        bufBody.innerHTML = '<tr><td colspan="4" style="color:var(--muted);">Antrean offline kosong.</td></tr>';
      } else {
        bufBody.innerHTML = state.offline_buffer.map(item => `
          <tr>
            <td>${item.raw.event_id}</td>
            <td>${item.raw.device_id}</td>
            <td>${item.raw.moisture_raw}% | ${item.raw.temperature_raw}°C | pH ${item.raw.ph_raw} | ${item.raw.ec_raw} mS/cm</td>
            <td style="color:var(--amber);">${item.reason}</td>
          </tr>
        `).join('');
      }

      // Tabel M3 Sink
      const m3Body = document.getElementById('m3TableBody');
      if (state.m3_sink_records.length === 0) {
        m3Body.innerHTML = '<tr><td colspan="5" style="color:var(--muted);">Belum ada data di ReadingSink M3.</td></tr>';
      } else {
        m3Body.innerHTML = state.m3_sink_records.slice().reverse().map(env => {
          const r = env.reading;
          return `
            <tr>
              <td>${r.ts}</td>
              <td><strong>${r.device_id}</strong><br/><span style="color:var(--muted);">${r.plot_id} (${env.plot_meta.plant})</span></td>
              <td>${r.moisture_pct.toFixed(1)}% | ${r.temperature_c.toFixed(1)}°C | pH ${r.ph.toFixed(2)} | ${r.ec_ms_cm.toFixed(2)}</td>
              <td><span class="pill ${r.quality}">${r.quality.toUpperCase()}</span></td>
              <td>${env.buzzer_alarm_active ? '🚨 ON' : 'OFF'}</td>
            </tr>
          `;
        }).join('');

        document.getElementById('m3JsonPreview').textContent = JSON.stringify({
          soil_reading: lastRecord.reading,
          plot_meta: lastRecord.plot_meta,
          anomalies: lastRecord.anomalies,
          buzzer_alarm_active: lastRecord.buzzer_alarm_active
        }, null, 2);
      }
    }

    function applyPreset(type) {
      const tsInput = document.getElementById('inTsSec');
      if (type !== 'spam') {
        tsInput.value = parseInt(tsInput.value || '1000', 10) + 30;
      }
      if (type === 'normal') {
        document.getElementById('inMoisture').value = 68.5;
        document.getElementById('inTemp').value = 28.4;
        document.getElementById('inPh').value = 4.5;
        document.getElementById('inEc').value = 1.2;
      } else if (type === 'suspect') {
        document.getElementById('inMoisture').value = 18.0;
        document.getElementById('inTemp').value = 43.5;
        document.getElementById('inPh').value = 2.8;
        document.getElementById('inEc').value = 4.5;
      } else if (type === 'bad') {
        document.getElementById('inMoisture').value = 145.0;
        document.getElementById('inTemp').value = 95.0;
        document.getElementById('inPh').value = -2.0;
        document.getElementById('inEc').value = 25.0;
      }
    }

    async function sendSensorData() {
      const payload = {
        event_id: `evt-gambut-${Date.now()}-${eventCounter++}`,
        device_id: document.getElementById('inDevice').value,
        ts: new Date().toISOString(),
        timestamp_sec: parseInt(document.getElementById('inTsSec').value, 10),
        moisture_raw: parseFloat(document.getElementById('inMoisture').value),
        temperature_raw: parseFloat(document.getElementById('inTemp').value),
        ph_raw: parseFloat(document.getElementById('inPh').value),
        ec_raw: parseFloat(document.getElementById('inEc').value)
      };

      const res = await fetch('/api/sensor/data', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-API-Key': API_KEY },
        body: JSON.stringify(payload)
      });
      const data = await res.json();
      const box = document.getElementById('sensorResult');
      box.className = 'alert-box ' + (res.status === 200 ? 'ok' : (res.status === 202 ? 'warn' : 'err'));
      box.textContent = `[HTTP ${res.status}] ${data.message}`;

      // Otomatis majukan waktu +20 detik supaya klik berikutnya tidak kena blokir interval (kecuali uji spam)
      if (res.status === 200 || res.status === 202) {
        document.getElementById('inTsSec').value = payload.timestamp_sec + 20;
      }
      await loadState();
    }

    async function toggleNetwork(online) {
      const res = await fetch('/api/network/mode', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-API-Key': API_KEY },
        body: JSON.stringify({ online })
      });
      const data = await res.json();
      const box = document.getElementById('bufferStatus');
      box.className = 'alert-box ' + (online ? 'ok' : 'warn');
      box.textContent = data.message;
      await loadState();
    }

    async function testRbacAction() {
      const [username, role] = document.getElementById('rbacUser').value.split('|');
      const payload = {
        username,
        role,
        aksi: document.getElementById('rbacAction').value,
        device_id: document.getElementById('rbacDevice').value,
        new_interval_sec: parseInt(document.getElementById('rbacInterval').value, 10)
      };
      const res = await fetch('/api/rbac/action', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-API-Key': API_KEY },
        body: JSON.stringify(payload)
      });
      const data = await res.json();
      const box = document.getElementById('rbacResult');
      box.className = 'alert-box ' + (data.success ? 'ok' : 'err');
      box.textContent = `[HTTP ${res.status}] ${data.message}`;
      await loadState();
    }

    function showAddDeviceModal() {
      const el = document.getElementById('addDeviceBox');
      el.style.display = el.style.display === 'none' ? 'block' : 'none';
    }

    async function registerNewDevice() {
      const payload = {
        device_id: document.getElementById('newDevId').value,
        nama: `Node Sensor ${document.getElementById('newPlotName').value}`,
        tipe: "ESP32-DOIT + SIM7000E + ADS1115 + DS18B20 + DFRobot",
        owner_username: document.getElementById('newOwner').value,
        plot_meta: {
          plot_id: document.getElementById('newPlotId').value,
          name: document.getElementById('newPlotName').value,
          latitude: -2.2200,
          longitude: 113.9300,
          area_ha: parseFloat(document.getElementById('newArea').value),
          plant: document.getElementById('newPlant').value
        }
      };
      const res = await fetch('/api/devices/register', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-API-Key': API_KEY },
        body: JSON.stringify(payload)
      });
      const data = await res.json();
      const box = document.getElementById('rbacResult');
      box.className = 'alert-box ' + (data.success ? 'ok' : 'err');
      box.textContent = data.message;
      if (data.success) document.getElementById('addDeviceBox').style.display = 'none';
      await loadState();
    }

    loadState();
  </script>
</body>
</html>
"##;
