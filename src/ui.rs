//! Antarmuka Web Sederhana untuk Modul 2 (IoT Communication, Edge Processing, & RBAC).
//!
//! Disajikan oleh Axum pada rute `GET /`.

pub const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Modul 2 - Smart Soil IoT</title>
  <style>
    * { box-sizing: border-box; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; }
    body {
      background-color: #e6eff8;
      color: #1e3a5f;
      margin: 0;
      padding: 20px;
      font-size: 14px;
      line-height: 1.45;
    }
    .container { max-width: 1120px; margin: 0 auto; }
    .header-box {
      background: #3b6998;
      color: #ffffff;
      border: 1px solid #2d547d;
      border-radius: 6px;
      padding: 16px 20px;
      margin-bottom: 16px;
    }
    .header-box h1 { margin: 0 0 5px 0; font-size: 20px; color: #ffffff; }
    .header-box p { margin: 0; color: #dce8f5; font-size: 13px; }
    .status-bar {
      margin-top: 12px;
      padding: 8px 12px;
      background: #f2f7fc;
      color: #1e3a5f;
      border-radius: 4px;
      display: flex;
      gap: 22px;
      flex-wrap: wrap;
      font-size: 13px;
    }
    .status-item strong { color: #16325c; }
    .grid-2 {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 16px;
      margin-bottom: 16px;
    }
    @media (max-width: 850px) { .grid-2 { grid-template-columns: 1fr; } }
    .panel {
      background: #f7faff;
      border: 1px solid #bdd1e6;
      border-radius: 6px;
      padding: 16px;
    }
    .panel h2 {
      margin: 0 0 5px 0;
      font-size: 15px;
      border-bottom: 1px solid #cfe0f2;
      padding-bottom: 7px;
      color: #234975;
    }
    .panel .sub { font-size: 12px; color: #527196; margin-bottom: 12px; }
    .form-row {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 10px;
      margin-bottom: 10px;
    }
    .field label {
      display: block;
      font-size: 12px;
      font-weight: 600;
      margin-bottom: 3px;
      color: #2c5282;
    }
    input, select {
      width: 100%;
      padding: 6px 8px;
      border: 1px solid #a8c3df;
      border-radius: 4px;
      background: #ffffff;
      color: #1e3a5f;
      font-size: 13px;
    }
    input:focus, select:focus {
      outline: none;
      border-color: #4a7eb5;
    }
    .btn-group {
      display: flex;
      gap: 6px;
      flex-wrap: wrap;
      margin-top: 8px;
      margin-bottom: 8px;
    }
    button {
      padding: 6px 12px;
      border: 1px solid #9cb9d9;
      border-radius: 4px;
      background: #dceaf7;
      color: #1e3a5f;
      cursor: pointer;
      font-size: 13px;
      font-weight: 500;
    }
    button:hover { background: #cbe0f3; }
    .btn-main {
      background: #3b6998;
      color: #ffffff;
      border-color: #2d547d;
    }
    .btn-main:hover { background: #2f5680; }
    .info-box {
      margin-top: 10px;
      padding: 8px 10px;
      background: #eaf2fb;
      border: 1px solid #bdd1e6;
      border-radius: 4px;
      font-size: 13px;
      color: #234975;
      white-space: pre-wrap;
    }
    .info-box.ok { background: #e6f4ea; border-color: #8cc59e; color: #1b5e30; }
    .info-box.err { background: #fce8e8; border-color: #dfa3a3; color: #8c2323; }
    .info-box.warn { background: #fef6e0; border-color: #dfc478; color: #6e510b; }
    table {
      width: 100%;
      border-collapse: collapse;
      font-size: 12px;
      margin-top: 8px;
      background: #ffffff;
    }
    th, td {
      border: 1px solid #c5d8ec;
      padding: 6px 8px;
      text-align: left;
    }
    th {
      background: #dceaf7;
      color: #1e3a5f;
      font-weight: 600;
    }
    pre {
      background: #edf4fc;
      border: 1px solid #bdd1e6;
      border-radius: 4px;
      padding: 10px;
      font-family: Consolas, monospace;
      font-size: 12px;
      color: #1b385c;
      overflow-x: auto;
      max-height: 240px;
      margin-top: 6px;
    }
  </style>
</head>
<body>
  <div class="container">
    <div class="header-box">
      <h1>Smart Soil IoT - Modul 2 (Gateway, Edge Processing & RBAC)</h1>
      <p>Pemrograman Fungsional (Rust + Axum) | Integrasi Modul 1 (Sensor Tanah Gambut) ke Modul 3 (ReadingSink & PlotMeta)</p>
      <div class="status-bar">
        <div class="status-item">Status Jaringan: <strong id="netBadge">ONLINE</strong></div>
        <div class="status-item">Perangkat Terdaftar: <strong id="kpiDevices">0</strong></div>
        <div class="status-item">Antrean Offline Buffer: <strong id="kpiBuffer">0</strong></div>
        <div class="status-item">Data Masuk ke Modul 3: <strong id="kpiM3">0</strong></div>
        <div class="status-item">Status Buzzer 5V: <strong id="kpiBuzzer">MATI (AMAN)</strong></div>
      </div>
    </div>

    <div class="grid-2">
      <!-- BAGIAN 1 -->
      <div class="panel">
        <h2>1. Input Data Sensor Edge & Kontrol Interval (anomaly.rs)</h2>
        <div class="sub">Validasi 4 parameter sensor tanah gambut (Moisture, Suhu, pH, EC) dan pemeriksaan interval sampling.</div>

        <div class="btn-group">
          <button onclick="applyPreset('normal')">Isi Data Normal</button>
          <button onclick="applyPreset('suspect')">Isi Data Kering (Suspect)</button>
          <button onclick="applyPreset('bad')">Isi Data Sensor Rusak (Bad)</button>
          <button onclick="applyPreset('spam')">Uji Interval Belum Tercapai</button>
        </div>

        <div class="form-row">
          <div class="field">
            <label>ID Perangkat (device_id)</label>
            <select id="inDevice"></select>
          </div>
          <div class="field">
            <label>Waktu Detik (timestamp_sec)</label>
            <input type="number" id="inTsSec" value="1000" />
          </div>
          <div class="field">
            <label>Moisture (%)</label>
            <input type="number" step="0.1" id="inMoisture" value="68.5" />
          </div>
          <div class="field">
            <label>Suhu Tanah (C)</label>
            <input type="number" step="0.1" id="inTemp" value="28.4" />
          </div>
          <div class="field">
            <label>pH Tanah (0-14)</label>
            <input type="number" step="0.1" id="inPh" value="4.5" />
          </div>
          <div class="field">
            <label>EC (mS/cm)</label>
            <input type="number" step="0.1" id="inEc" value="1.2" />
          </div>
        </div>

        <div class="btn-group">
          <button class="btn-main" onclick="sendSensorData()">Kirim Data Sensor (POST /api/sensor/data)</button>
        </div>

        <div id="sensorResult" class="info-box">Belum ada pengiriman data.</div>
      </div>

      <!-- BAGIAN 2 -->
      <div class="panel">
        <h2>2. Simulasi Koneksi & Offline Buffering (retry.rs)</h2>
        <div class="sub">Menyimpan data ke antrean buffer secara immutable saat koneksi mati, lalu mengirim ulang saat koneksi pulih.</div>

        <div class="btn-group">
          <button onclick="toggleNetwork(false)">Matikan Koneksi (Mode OFFLINE)</button>
          <button class="btn-main" onclick="toggleNetwork(true)">Sambungkan Kembali & Kirim Buffer (Mode ONLINE)</button>
        </div>

        <div id="bufferStatus" class="info-box">Mode koneksi saat ini: ONLINE</div>

        <div style="margin-top: 10px; font-weight: 600; font-size: 12px; color: #2c5282;">Tabel Antrean Offline Buffer:</div>
        <table>
          <thead>
            <tr>
              <th>Event ID</th>
              <th>Device ID</th>
              <th>Nilai Sensor (Moisture / Suhu / pH / EC)</th>
              <th>Keterangan</th>
            </tr>
          </thead>
          <tbody id="bufferTableBody">
            <tr><td colspan="4">Antrean kosong.</td></tr>
          </tbody>
        </table>
      </div>
    </div>

    <div class="grid-2">
      <!-- BAGIAN 3 -->
      <div class="panel">
        <h2>3. Registrasi Perangkat, Pemetaan Pemilik & RBAC (device, user, rbac)</h2>
        <div class="sub">Pemetaan perangkat ke pemilik dan PlotMeta, serta validasi hak akses Admin, Operator, dan Viewer.</div>

        <table>
          <thead>
            <tr>
              <th>Device ID</th>
              <th>Pemilik</th>
              <th>Plot ID & Tanaman (PlotMeta)</th>
              <th>Interval</th>
            </tr>
          </thead>
          <tbody id="deviceTableBody"></tbody>
        </table>

        <div style="margin-top: 12px; font-weight: 600; font-size: 12px; color: #2c5282;">Uji Hak Akses (RBAC) & Kepemilikan Perangkat:</div>
        <div class="form-row" style="margin-top: 6px;">
          <div class="field">
            <label>Pengguna & Role</label>
            <select id="rbacUser">
              <option value="admin_utama|Admin">admin_utama (Role: Admin)</option>
              <option value="petani_budi|Operator">petani_budi (Role: Operator - Pemilik ESP32-GAMBUT-01)</option>
              <option value="petani_siti|Operator">petani_siti (Role: Operator - Pemilik ESP32-GAMBUT-02)</option>
              <option value="peneliti_rina|Viewer">peneliti_rina (Role: Viewer - Hanya Baca)</option>
            </select>
          </div>
          <div class="field">
            <label>Perangkat Tujuan</label>
            <select id="rbacDevice"></select>
          </div>
          <div class="field">
            <label>Aksi</label>
            <select id="rbacAction">
              <option value="Kontrol">Kontrol (Ubah Interval Sampling)</option>
              <option value="Baca">Baca (Lihat Data)</option>
              <option value="Hapus">Hapus (Hapus Perangkat)</option>
            </select>
          </div>
          <div class="field">
            <label>Interval Baru (detik)</label>
            <input type="number" id="rbacInterval" value="30" />
          </div>
        </div>

        <div class="btn-group">
          <button class="btn-main" onclick="testRbacAction()">Uji Validasi Akses</button>
          <button onclick="showAddDeviceModal()">Tambah Perangkat Baru</button>
        </div>

        <div id="rbacResult" class="info-box">Pilih pengguna dan aksi untuk menguji validasi akses.</div>

        <div id="addDeviceBox" style="display:none; margin-top:12px; padding-top:10px; border-top:1px solid #cfe0f2;">
          <div style="font-weight:600; font-size:12px; margin-bottom:6px; color:#2c5282;">Form Tambah Perangkat & PlotMeta:</div>
          <div class="form-row">
            <div class="field"><label>Device ID</label><input id="newDevId" value="ESP32-GAMBUT-03" /></div>
            <div class="field"><label>Username Pemilik</label><input id="newOwner" value="petani_budi" /></div>
            <div class="field"><label>Plot ID</label><input id="newPlotId" value="PLOT-KALTENG-03" /></div>
            <div class="field"><label>Nama Petak</label><input id="newPlotName" value="Petak Gambut Blok C" /></div>
            <div class="field"><label>Tanaman (plant)</label><input id="newPlant" value="nanas" /></div>
            <div class="field"><label>Luas Lahan (area_ha)</label><input type="number" step="0.1" id="newArea" value="1.8" /></div>
          </div>
          <button class="btn-main" onclick="registerNewDevice()">Simpan Perangkat</button>
        </div>
      </div>

      <!-- BAGIAN 4 -->
      <div class="panel">
        <h2>4. Output Kontrak ke Modul 3 (contracts.rs & pipeline.rs)</h2>
        <div class="sub">Data tervalidasi yang dikirim ke ReadingSink Modul 3 dalam format SoilReading dan PlotMeta.</div>

        <table>
          <thead>
            <tr>
              <th>Waktu (ts)</th>
              <th>Device & Plot</th>
              <th>Moisture / Suhu / pH / EC</th>
              <th>Quality</th>
              <th>Buzzer</th>
            </tr>
          </thead>
          <tbody id="m3TableBody">
            <tr><td colspan="5">Belum ada data yang dikirim ke Modul 3.</td></tr>
          </tbody>
        </table>

        <div style="margin-top: 10px; font-weight: 600; font-size: 12px; color: #2c5282;">Data JSON Terakhir (SoilReading & PlotMeta):</div>
        <pre id="m3JsonPreview">// Kirim data sensor pada Bagian 1 untuk melihat output JSON di sini</pre>
      </div>
    </div>
  </div>

  <script>
    const API_KEY = "TubesPF-demo-key";
    let eventCounter = 1;

    async function loadState() {
      const res = await fetch('/api/state');
      const state = await res.json();

      const netBadge = document.getElementById('netBadge');
      if (state.network_online) {
        netBadge.textContent = 'ONLINE';
        netBadge.style.color = '#1b5e30';
      } else {
        netBadge.textContent = 'OFFLINE (Simpan ke Buffer)';
        netBadge.style.color = '#8c2323';
      }

      const devices = state.devices || [];
      document.getElementById('kpiDevices').textContent = devices.length;
      document.getElementById('kpiBuffer').textContent = state.offline_buffer.length;
      document.getElementById('kpiM3').textContent = state.m3_sink_records.length;

      const lastRecord = state.m3_sink_records[state.m3_sink_records.length - 1];
      const kpiBuzzer = document.getElementById('kpiBuzzer');
      if (lastRecord && lastRecord.buzzer_alarm_active) {
        kpiBuzzer.textContent = 'MENYALA (SENSOR ANOMALI KRITIS)';
        kpiBuzzer.style.color = '#8c2323';
      } else {
        kpiBuzzer.textContent = 'MATI (AMAN)';
        kpiBuzzer.style.color = '#1b5e30';
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
        inDev.innerHTML += `<option value="${d.id}">${d.id} (Interval: ${d.sampling_interval_sec}s)</option>`;
        rbacDev.innerHTML += `<option value="${d.id}">${d.id}</option>`;
      });
      if (prevDev) inDev.value = prevDev;
      if (prevRbacDev) rbacDev.value = prevRbacDev;

      // Update daftar Pengguna & Role secara dinamis berdasarkan pemilik perangkat terdaftar
      const ownerMap = {};
      devices.forEach(d => {
        if (d.owner) {
          if (!ownerMap[d.owner]) ownerMap[d.owner] = [];
          ownerMap[d.owner].push(d.id);
        }
      });
      let userOptions = `<option value="admin_utama|Admin">admin_utama (Role: Admin)</option>`;
      Object.keys(ownerMap).forEach(owner => {
        const ownedList = ownerMap[owner].join(', ');
        userOptions += `<option value="${owner}|Operator">${owner} (Role: Operator - Pemilik ${ownedList})</option>`;
      });
      userOptions += `<option value="peneliti_rina|Viewer">peneliti_rina (Role: Viewer - Hanya Baca)</option>`;
      rbacUser.innerHTML = userOptions;
      if (prevRbacUser && Array.from(rbacUser.options).some(o => o.value === prevRbacUser)) {
        rbacUser.value = prevRbacUser;
      }

      const devBody = document.getElementById('deviceTableBody');
      devBody.innerHTML = devices.map(d => `
        <tr>
          <td><strong>${d.id}</strong></td>
          <td>${d.owner || '-'}</td>
          <td>${d.plot ? `${d.plot.plot_id} (${d.plot.plant}, ${d.plot.area_ha} ha)` : '-'}</td>
          <td>${d.sampling_interval_sec} detik</td>
        </tr>
      `).join('');

      const bufBody = document.getElementById('bufferTableBody');
      if (state.offline_buffer.length === 0) {
        bufBody.innerHTML = '<tr><td colspan="4">Antrean kosong.</td></tr>';
      } else {
        bufBody.innerHTML = state.offline_buffer.map(item => `
          <tr>
            <td>${item.raw.event_id}</td>
            <td>${item.raw.device_id}</td>
            <td>${item.raw.moisture_raw}% | ${item.raw.temperature_raw} C | pH ${item.raw.ph_raw} | ${item.raw.ec_raw} mS/cm</td>
            <td>${item.reason}</td>
          </tr>
        `).join('');
      }

      const m3Body = document.getElementById('m3TableBody');
      if (state.m3_sink_records.length === 0) {
        m3Body.innerHTML = '<tr><td colspan="5">Belum ada data yang dikirim ke Modul 3.</td></tr>';
      } else {
        m3Body.innerHTML = state.m3_sink_records.slice().reverse().map(env => {
          const r = env.reading;
          return `
            <tr>
              <td>${r.ts}</td>
              <td><strong>${r.device_id}</strong><br/>${r.plot_id} (${env.plot_meta.plant})</td>
              <td>${r.moisture_pct.toFixed(1)}% | ${r.temperature_c.toFixed(1)} C | pH ${r.ph.toFixed(2)} | ${r.ec_ms_cm.toFixed(2)}</td>
              <td><strong>${r.quality.toUpperCase()}</strong></td>
              <td>${env.buzzer_alarm_active ? 'AKTIF' : 'MATI'}</td>
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
      box.className = 'info-box ' + (res.status === 200 ? 'ok' : (res.status === 202 ? 'warn' : 'err'));
      box.textContent = `[HTTP ${res.status}] ${data.message}`;

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
