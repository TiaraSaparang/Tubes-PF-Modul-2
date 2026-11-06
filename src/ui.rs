//! Antarmuka Web Dashboard untuk Modul 2 (Gateway, Edge Processing & RBAC).
//!
//! Disajikan oleh Axum pada rute `GET /`.

pub const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Smart Soil IoT - Modul 2 Dashboard</title>
  <style>
    * { box-sizing: border-box; font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif; }
    body {
      background-color: #edf3f8;
      color: #1e3a5f;
      margin: 0;
      padding: 16px;
      font-size: 13px;
    }
    .container { max-width: 1140px; margin: 0 auto; }
    
    /* Top Header & Status */
    .header-card {
      background: #2b5784;
      color: #ffffff;
      border-radius: 8px;
      padding: 14px 18px;
      margin-bottom: 14px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.08);
    }
    .header-card h1 {
      margin: 0 0 10px 0;
      font-size: 18px;
      font-weight: 600;
      letter-spacing: 0.3px;
    }
    .kpi-row {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
    }
    .kpi-badge {
      background: #ffffff;
      color: #1e3a5f;
      padding: 6px 12px;
      border-radius: 5px;
      font-size: 12px;
      display: flex;
      align-items: center;
      gap: 6px;
      box-shadow: 0 1px 2px rgba(0,0,0,0.05);
    }
    .kpi-badge strong { font-weight: 700; color: #16325c; }
    
    /* Grid Layout */
    .grid-2 {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 14px;
      margin-bottom: 14px;
    }
    @media (max-width: 860px) { .grid-2 { grid-template-columns: 1fr; } }
    
    /* Card Panel */
    .card {
      background: #ffffff;
      border: 1px solid #c9dcf0;
      border-radius: 8px;
      padding: 14px 16px;
      box-shadow: 0 1px 3px rgba(0,0,0,0.04);
    }
    .card-title {
      font-size: 14px;
      font-weight: 700;
      color: #234975;
      margin: 0 0 12px 0;
      padding-bottom: 6px;
      border-bottom: 2px solid #e1ecf7;
    }
    
    /* Forms & Inputs */
    .form-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 10px;
      margin-bottom: 12px;
    }
    .field {
      display: flex;
      flex-direction: column;
    }
    .field label {
      font-size: 11px;
      font-weight: 600;
      color: #3b5a80;
      margin-bottom: 4px;
      text-transform: uppercase;
      letter-spacing: 0.4px;
    }
    input, select {
      width: 100%;
      padding: 7px 9px;
      border: 1px solid #b3cce6;
      border-radius: 5px;
      background: #fbfdff;
      color: #1e3a5f;
      font-size: 13px;
    }
    input:focus, select:focus {
      outline: none;
      border-color: #3b6998;
      background: #ffffff;
      box-shadow: 0 0 0 2px rgba(59, 105, 152, 0.15);
    }
    
    /* Buttons */
    .btn-row {
      display: flex;
      flex-wrap: wrap;
      gap: 6px;
      margin-bottom: 10px;
    }
    button {
      padding: 6px 12px;
      border: 1px solid #a8c5e2;
      border-radius: 5px;
      background: #e9f2fa;
      color: #20436b;
      cursor: pointer;
      font-size: 12px;
      font-weight: 600;
      transition: all 0.15s ease;
    }
    button:hover {
      background: #d5e6f6;
      border-color: #8bb3dc;
    }
    .btn-primary {
      background: #2b5784;
      color: #ffffff;
      border-color: #20436b;
      padding: 8px 16px;
      font-size: 13px;
    }
    .btn-primary:hover {
      background: #20436b;
    }
    .btn-danger {
      background: #faeaea;
      color: #8c2323;
      border-color: #e5b3b3;
    }
    .btn-danger:hover {
      background: #f5d4d4;
    }
    
    /* Info Box */
    .info-box {
      margin-top: 10px;
      padding: 8px 10px;
      border-radius: 5px;
      font-size: 12px;
      border: 1px solid #bdd1e6;
      background: #f3f8fd;
      color: #234975;
      line-height: 1.4;
    }
    .info-box.ok { background: #eaf6ec; border-color: #9cd1a8; color: #1e6333; }
    .info-box.warn { background: #fdf7e7; border-color: #e5ce8a; color: #7a5a08; }
    .info-box.err { background: #fdf0f0; border-color: #e8a9a9; color: #9c2525; }
    
    /* Tables */
    table {
      width: 100%;
      border-collapse: collapse;
      font-size: 12px;
      margin-top: 6px;
    }
    th, td {
      border: 1px solid #d4e3f2;
      padding: 6px 8px;
      text-align: left;
    }
    th {
      background: #edf4fa;
      color: #234975;
      font-weight: 600;
    }
    tbody tr:nth-child(even) { background: #f9fcfe; }
    
    /* Code preview */
    pre {
      background: #f4f8fc;
      border: 1px solid #c9dcf0;
      border-radius: 5px;
      padding: 8px 10px;
      font-family: Consolas, monospace;
      font-size: 11px;
      color: #1b385c;
      overflow-x: auto;
      max-height: 220px;
      margin: 8px 0 0 0;
    }
  </style>
</head>
<body>
  <div class="container">
    <!-- TOP STATUS BAR -->
    <div class="header-card">
      <h1>Smart Soil IoT — Dashboard Modul 2</h1>
      <div class="kpi-row">
        <div class="kpi-badge">Koneksi: <strong id="netBadge">ONLINE</strong></div>
        <div class="kpi-badge">Perangkat: <strong id="kpiDevices">0</strong></div>
        <div class="kpi-badge">Antrean Buffer: <strong id="kpiBuffer">0</strong></div>
        <div class="kpi-badge">Terkirim ke M3: <strong id="kpiM3">0</strong></div>
        <div class="kpi-badge">Buzzer 5V: <strong id="kpiBuzzer">MATI (AMAN)</strong></div>
      </div>
    </div>

    <div class="grid-2">
      <!-- PANEL 1: INPUT SENSOR -->
      <div class="card">
        <div class="card-title">1. Input Data Sensor Tanah</div>
        
        <div class="btn-row">
          <button onclick="applyPreset('normal')">Preset Normal</button>
          <button onclick="applyPreset('suspect')">Preset Kering (Suspect)</button>
          <button onclick="applyPreset('bad')">Preset Rusak (Bad)</button>
          <button onclick="applyPreset('spam')">Uji Interval Cepat</button>
        </div>

        <div class="form-grid">
          <div class="field">
            <label>Perangkat (Device ID)</label>
            <select id="inDevice"></select>
          </div>
          <div class="field">
            <label>Waktu (Detik)</label>
            <input type="number" id="inTsSec" value="1000" />
          </div>
          <div class="field">
            <label>Kelembapan Tanah (%)</label>
            <input type="number" step="0.1" id="inMoisture" value="68.5" />
          </div>
          <div class="field">
            <label>Suhu Tanah (&deg;C)</label>
            <input type="number" step="0.1" id="inTemp" value="28.4" />
          </div>
          <div class="field">
            <label>pH Tanah (0-14)</label>
            <input type="number" step="0.1" id="inPh" value="4.5" />
          </div>
          <div class="field">
            <label>EC Pupuk (mS/cm)</label>
            <input type="number" step="0.1" id="inEc" value="1.2" />
          </div>
        </div>

        <button class="btn-primary" style="width:100%;" onclick="sendSensorData()">Kirim Data Sensor</button>
        <div id="sensorResult" class="info-box">Siap mengirimkan data sensor.</div>
      </div>

      <!-- PANEL 2: SIMULASI JARINGAN & BUFFER -->
      <div class="card">
        <div class="card-title">2. Kontrol Jaringan & Offline Buffer</div>
        
        <div class="btn-row">
          <button class="btn-danger" onclick="toggleNetwork(false)">Putus Sinyal (Mode Offline)</button>
          <button class="btn-primary" onclick="toggleNetwork(true)">Pulihkan Sinyal & Kirim Buffer (Online)</button>
        </div>

        <div id="bufferStatus" class="info-box">Mode koneksi saat ini: ONLINE</div>

        <table style="margin-top:10px;">
          <thead>
            <tr>
              <th>Event ID</th>
              <th>Device</th>
              <th>Nilai Sensor</th>
              <th>Keterangan</th>
            </tr>
          </thead>
          <tbody id="bufferTableBody">
            <tr><td colspan="4">Antrean buffer kosong.</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="grid-2">
      <!-- PANEL 3: HAK AKSES & REGISTRASI -->
      <div class="card">
        <div class="card-title">3. Pengaturan Hak Akses (RBAC) & Perangkat</div>
        
        <table>
          <thead>
            <tr>
              <th>Device ID</th>
              <th>Pemilik</th>
              <th>Plot & Tanaman</th>
              <th>Interval</th>
            </tr>
          </thead>
          <tbody id="deviceTableBody"></tbody>
        </table>

        <div class="form-grid" style="margin-top:12px;">
          <div class="field">
            <label>Pengguna & Peran</label>
            <select id="rbacUser">
              <option value="admin_utama|Admin">admin_utama (Admin)</option>
              <option value="petani_budi|Operator">petani_budi (Operator)</option>
              <option value="petani_siti|Operator">petani_siti (Operator)</option>
              <option value="peneliti_rina|Viewer">peneliti_rina (Viewer)</option>
            </select>
          </div>
          <div class="field">
            <label>Perangkat Target</label>
            <select id="rbacDevice"></select>
          </div>
          <div class="field">
            <label>Aksi</label>
            <select id="rbacAction">
              <option value="Kontrol">Kontrol (Ubah Interval)</option>
              <option value="Baca">Baca Data</option>
              <option value="Hapus">Hapus Perangkat</option>
            </select>
          </div>
          <div class="field">
            <label>Interval Baru (detik)</label>
            <input type="number" id="rbacInterval" value="30" />
          </div>
        </div>

        <div class="btn-row">
          <button class="btn-primary" onclick="testRbacAction()">Jalankan Aksi</button>
          <button onclick="showAddDeviceModal()">+ Tambah Perangkat</button>
        </div>

        <div id="rbacResult" class="info-box">Pilih pengguna dan aksi, lalu klik Jalankan Aksi.</div>

        <div id="addDeviceBox" style="display:none; margin-top:10px; padding:10px; background:#f4f9fd; border-radius:5px; border:1px solid #cce0f2;">
          <div style="font-weight:700; margin-bottom:8px; font-size:12px; color:#234975;">Tambah Perangkat & Lahan:</div>
          <div class="form-grid">
            <div class="field"><label>Device ID</label><input id="newDevId" value="ESP32-GAMBUT-03" /></div>
            <div class="field"><label>Pemilik</label><input id="newOwner" value="petani_budi" /></div>
            <div class="field"><label>Plot ID</label><input id="newPlotId" value="PLOT-KALTENG-03" /></div>
            <div class="field"><label>Nama Lahan</label><input id="newPlotName" value="Petak Gambut Blok C" /></div>
            <div class="field"><label>Tanaman</label><input id="newPlant" value="nanas" /></div>
            <div class="field"><label>Luas (Ha)</label><input type="number" step="0.1" id="newArea" value="1.8" /></div>
          </div>
          <button class="btn-primary" style="width:100%; margin-top:6px;" onclick="registerNewDevice()">Simpan Perangkat</button>
        </div>
      </div>

      <!-- PANEL 4: DATA OUTPUT MODUL 3 -->
      <div class="card">
        <div class="card-title">4. Data Masuk ke Modul 3 (ReadingSink)</div>
        
        <table>
          <thead>
            <tr>
              <th>Waktu</th>
              <th>Perangkat</th>
              <th>Nilai Sensor</th>
              <th>Status</th>
              <th>Buzzer</th>
            </tr>
          </thead>
          <tbody id="m3TableBody">
            <tr><td colspan="5">Belum ada data masuk ke Modul 3.</td></tr>
          </tbody>
        </table>

        <div style="margin-top:10px; font-size:11px; font-weight:700; color:#3b5a80; text-transform:uppercase;">Format JSON Terakhir:</div>
        <pre id="m3JsonPreview">// Output JSON data terakhir akan tampil di sini</pre>
      </div>
    </div>
  </div>

  <script>
    const API_KEY = "TubesPF-demo-key";
    let eventCounter = 1;
    let lastAcceptedTsByDevice = {};

    async function loadState() {
      const res = await fetch('/api/state');
      const state = await res.json();

      const netBadge = document.getElementById('netBadge');
      if (state.network_online) {
        netBadge.textContent = 'ONLINE';
        netBadge.style.color = '#1e6333';
      } else {
        netBadge.textContent = 'OFFLINE';
        netBadge.style.color = '#9c2525';
      }

      const devices = state.devices || [];
      document.getElementById('kpiDevices').textContent = devices.length;
      document.getElementById('kpiBuffer').textContent = state.offline_buffer.length;
      document.getElementById('kpiM3').textContent = state.m3_sink_records.length;

      const lastRecord = state.m3_sink_records[state.m3_sink_records.length - 1];
      const kpiBuzzer = document.getElementById('kpiBuzzer');
      if (lastRecord && lastRecord.buzzer_alarm_active) {
        kpiBuzzer.textContent = 'MENYALA (BAHAYA)';
        kpiBuzzer.style.color = '#9c2525';
      } else {
        kpiBuzzer.textContent = 'MATI (AMAN)';
        kpiBuzzer.style.color = '#1e6333';
      }

      const inDev = document.getElementById('inDevice');
      const rbacDev = document.getElementById('rbacDevice');
      const rbacUser = document.getElementById('rbacUser');
      const prevDev = inDev.value;
      const prevRbacDev = rbacDev.value;
      const prevRbacUser = rbacUser.value;
      inDev.innerHTML = '';
      rbacDev.innerHTML = '';
      devices.forEach(d => {
        inDev.innerHTML += `<option value="${d.id}">${d.id} (${d.sampling_interval_sec}s)</option>`;
        rbacDev.innerHTML += `<option value="${d.id}">${d.id}</option>`;
      });
      if (prevDev) inDev.value = prevDev;
      if (prevRbacDev) rbacDev.value = prevRbacDev;

      const ownerMap = {};
      devices.forEach(d => {
        if (d.owner) {
          if (!ownerMap[d.owner]) ownerMap[d.owner] = [];
          ownerMap[d.owner].push(d.id);
        }
      });
      let userOptions = `<option value="admin_utama|Admin">admin_utama (Admin)</option>`;
      Object.keys(ownerMap).forEach(owner => {
        const ownedList = ownerMap[owner].join(', ');
        userOptions += `<option value="${owner}|Operator">${owner} (Operator - ${ownedList})</option>`;
      });
      userOptions += `<option value="peneliti_rina|Viewer">peneliti_rina (Viewer)</option>`;
      rbacUser.innerHTML = userOptions;
      if (prevRbacUser && Array.from(rbacUser.options).some(o => o.value === prevRbacUser)) {
        rbacUser.value = prevRbacUser;
      }

      const devBody = document.getElementById('deviceTableBody');
      devBody.innerHTML = devices.map(d => `
        <tr>
          <td><strong>${d.id}</strong></td>
          <td>${d.owner || '-'}</td>
          <td>${d.plot ? `${d.plot.plot_id} (${d.plot.plant})` : '-'}</td>
          <td>${d.sampling_interval_sec}s</td>
        </tr>
      `).join('');

      const bufBody = document.getElementById('bufferTableBody');
      if (state.offline_buffer.length === 0) {
        bufBody.innerHTML = '<tr><td colspan="4">Antrean buffer kosong.</td></tr>';
      } else {
        bufBody.innerHTML = state.offline_buffer.map(item => `
          <tr>
            <td>${item.raw.event_id}</td>
            <td>${item.raw.device_id}</td>
            <td>${item.raw.moisture_raw}% / ${item.raw.temperature_raw}&deg;C / pH ${item.raw.ph_raw} / ${item.raw.ec_raw}</td>
            <td>${item.reason}</td>
          </tr>
        `).join('');
      }

      const m3Body = document.getElementById('m3TableBody');
      if (state.m3_sink_records.length === 0) {
        m3Body.innerHTML = '<tr><td colspan="5">Belum ada data masuk ke Modul 3.</td></tr>';
      } else {
        m3Body.innerHTML = state.m3_sink_records.slice().reverse().map(env => {
          const r = env.reading;
          return `
            <tr>
              <td>${r.ts.substring(11, 19)}</td>
              <td><strong>${r.device_id}</strong> (${env.plot_meta.plant})</td>
              <td>${r.moisture_pct.toFixed(1)}% / ${r.temperature_c.toFixed(1)}&deg;C / pH ${r.ph.toFixed(1)} / ${r.ec_ms_cm.toFixed(1)}</td>
              <td><strong>${r.quality.toUpperCase()}</strong></td>
              <td>${env.buzzer_alarm_active ? '<span style="color:#9c2525;font-weight:700;">AKTIF</span>' : 'Aman'}</td>
            </tr>
          `;
        }).join('');

        document.getElementById('m3JsonPreview').textContent = JSON.stringify({
          soil_reading: lastRecord.reading,
          plot_meta: lastRecord.plot_meta,
          buzzer_active: lastRecord.buzzer_alarm_active
        }, null, 2);
      }
    }

    function applyPreset(type) {
      const tsInput = document.getElementById('inTsSec');
      const devId = document.getElementById('inDevice').value;
      const lastTs = lastAcceptedTsByDevice[devId] !== undefined ? lastAcceptedTsByDevice[devId] : 1000;
      const box = document.getElementById('sensorResult');

      if (type === 'spam') {
        tsInput.value = lastTs + 2;
        box.className = 'info-box warn';
        box.textContent = `Waktu diatur ke ${lastTs + 2}s (terlalu cepat). Klik "Kirim Data Sensor" untuk menguji batas interval.`;
        return;
      }

      tsInput.value = lastTs + 35;
      if (type === 'normal') {
        document.getElementById('inMoisture').value = 68.5;
        document.getElementById('inTemp').value = 28.4;
        document.getElementById('inPh').value = 4.5;
        document.getElementById('inEc').value = 1.2;
        box.className = 'info-box ok';
        box.textContent = 'Preset Normal dimuat. Klik "Kirim Data Sensor".';
      } else if (type === 'suspect') {
        document.getElementById('inMoisture').value = 18.0;
        document.getElementById('inTemp').value = 43.5;
        document.getElementById('inPh').value = 2.8;
        document.getElementById('inEc').value = 4.5;
        box.className = 'info-box warn';
        box.textContent = 'Preset Kering (Suspect) dimuat. Klik "Kirim Data Sensor".';
      } else if (type === 'bad') {
        document.getElementById('inMoisture').value = 145.0;
        document.getElementById('inTemp').value = 95.0;
        document.getElementById('inPh').value = -2.0;
        document.getElementById('inEc').value = 25.0;
        box.className = 'info-box err';
        box.textContent = 'Preset Rusak (Bad) dimuat. Klik "Kirim Data Sensor".';
      }
    }

    async function sendSensorData() {
      const payload = {
        event_id: `evt-${Date.now().toString().slice(-6)}-${eventCounter++}`,
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
      box.className = 'info-box ' + (res.status === 200 ? 'ok' : (res.status === 202 ? 'warn' : 'err'));
      box.textContent = `[${res.status}] ${data.message}`;

      if (res.status === 200) {
        lastAcceptedTsByDevice[payload.device_id] = payload.timestamp_sec;
        document.getElementById('inTsSec').value = payload.timestamp_sec + 35;
      } else if (res.status === 202) {
        document.getElementById('inTsSec').value = payload.timestamp_sec + 35;
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
      box.className = 'info-box ' + (online ? 'ok' : 'warn');
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
      box.className = 'info-box ' + (data.success ? 'ok' : 'err');
      box.textContent = `[${res.status}] ${data.message}`;
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
      box.className = 'info-box ' + (data.success ? 'ok' : 'err');
      box.textContent = data.message;
      if (data.success) document.getElementById('addDeviceBox').style.display = 'none';
      await loadState();
      if (data.success) {
        document.getElementById('rbacUser').value = `${payload.owner_username}|Operator`;
        document.getElementById('rbacDevice').value = payload.device_id;
      }
    }

    loadState();
  </script>
</body>
</html>
"##;
