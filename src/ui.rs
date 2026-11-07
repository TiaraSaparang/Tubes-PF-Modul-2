//! Antarmuka Web Dashboard untuk Modul 2 (Gateway, Edge Processing & RBAC).
//!
//! Disajikan oleh Axum pada rute `GET /`.

pub const DASHBOARD_HTML: &str = r##"<!DOCTYPE html>
<html lang="id">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Smart Soil IoT - Gateway & User Management Console</title>
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
    
    /* Header Card */
    .header-card {
      background: #2b5784;
      color: #ffffff;
      border-radius: 8px;
      padding: 14px 18px;
      margin-bottom: 14px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.08);
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 10px;
    }
    .header-title h1 {
      margin: 0 0 4px 0;
      font-size: 18px;
      font-weight: 600;
      letter-spacing: 0.3px;
    }
    .header-title p {
      margin: 0;
      font-size: 12px;
      color: #d1e2f3;
    }
    .user-profile-badge {
      background: #ffffff;
      color: #1e3a5f;
      padding: 6px 14px;
      border-radius: 20px;
      font-size: 12px;
      display: flex;
      align-items: center;
      gap: 10px;
      box-shadow: 0 1px 3px rgba(0,0,0,0.1);
    }
    .user-profile-badge strong { color: #16325c; }
    
    /* Status KPI Row */
    .kpi-row {
      display: flex;
      flex-wrap: wrap;
      gap: 10px;
      margin-bottom: 14px;
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
      border: 1px solid #c9dcf0;
    }
    .kpi-badge strong { font-weight: 700; color: #16325c; }
    
    /* Grid */
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
    
    /* Forms */
    .form-grid {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 10px;
      margin-bottom: 12px;
    }
    .field { display: flex; flex-direction: column; }
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
    button:hover { background: #d5e6f6; border-color: #8bb3dc; }
    .btn-primary {
      background: #2b5784;
      color: #ffffff;
      border-color: #20436b;
      padding: 7px 14px;
    }
    .btn-primary:hover { background: #20436b; }
    .btn-danger {
      background: #faeaea;
      color: #8c2323;
      border-color: #e5b3b3;
    }
    .btn-danger:hover { background: #f5d4d4; }
    
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
    th { background: #edf4fa; color: #234975; font-weight: 600; }
    tbody tr:nth-child(even) { background: #f9fcfe; }
    
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

    /* Auth Screen */
    .login-container {
      max-width: 460px;
      margin: 40px auto;
      background: #ffffff;
      border: 1px solid #c9dcf0;
      border-radius: 8px;
      padding: 24px;
      box-shadow: 0 4px 12px rgba(0,0,0,0.06);
    }
    .login-header {
      text-align: center;
      margin-bottom: 20px;
    }
    .login-header h2 { margin: 0 0 6px 0; font-size: 18px; color: #234975; }
    .login-header p { margin: 0; font-size: 12px; color: #527196; }
  </style>
</head>
<body>
  <div class="container">

    <!-- TOP HEADER -->
    <div class="header-card">
      <div class="header-title">
        <h1>Smart Soil IoT — Modul 2 Gateway & User Management</h1>
        <p>PBL FP 2026 | IoT Gateway, Edge Processing, Device Mapping & RBAC</p>
      </div>
      <div id="authProfileBox" class="user-profile-badge" style="display:none;">
        <span>👤 <strong id="currentUsername">-</strong> (<span id="currentUserRole">-</span>)</span>
        <button onclick="doLogout()" style="padding:3px 8px; font-size:11px;">Logout</button>
      </div>
    </div>

    <!-- VIEW 1: FORM LOGIN (JIKA BELUM LOGIN) -->
    <div id="loginSection" class="login-container">
      <div class="login-header">
        <h2>Autentikasi & Pengelolaan Peran Pengguna</h2>
        <p>Silakan login untuk mengakses konsol manajemen perangkat dan hak akses.</p>
      </div>

      <div class="field" style="margin-bottom:12px;">
        <label>Username</label>
        <input type="text" id="loginUser" value="petani_budi" placeholder="admin_utama / petani_budi / peneliti_rina" />
      </div>

      <div class="field" style="margin-bottom:14px;">
        <label>Password</label>
        <input type="password" id="loginPass" value="budi123" placeholder="admin123 / budi123 / rina123" />
      </div>

      <button class="btn-primary" style="width:100%; padding:9px; font-size:13px;" onclick="doLogin()">Masuk ke Sistem (Login Backend)</button>

      <div style="margin-top:16px; font-size:11px; color:#527196; text-align:center;">
        Tombol Cepat Pengujian Demo:
      </div>
      <div class="btn-row" style="justify-content:center; margin-top:8px;">
        <button onclick="quickFillLogin('admin_utama', 'admin123')">Login Admin</button>
        <button onclick="quickFillLogin('petani_budi', 'budi123')">Login Petani Budi (Operator)</button>
        <button onclick="quickFillLogin('peneliti_rina', 'rina123')">Login Peneliti Rina (Viewer)</button>
      </div>

      <div id="loginMsg" class="info-box" style="display:none;"></div>
    </div>

    <!-- VIEW 2: DASHBOARD GATEWAY (SETELAH LOGIN BERHASIL) -->
    <div id="dashboardSection" style="display:none;">
      
      <!-- KPI STATUS BAR -->
      <div class="kpi-row">
        <div class="kpi-badge">Koneksi Gateway: <strong id="netBadge">ONLINE</strong></div>
        <div class="kpi-badge">Perangkat Diakses: <strong id="kpiDevices">0</strong></div>
        <div class="kpi-badge">Antrean Buffer: <strong id="kpiBuffer">0</strong></div>
        <div class="kpi-badge">Terkirim ke Modul 3: <strong id="kpiM3">0</strong></div>
        <div class="kpi-badge">Buzzer 5V: <strong id="kpiBuzzer">MATI (AMAN)</strong></div>
      </div>

      <div class="grid-2">
        <!-- PANEL 1: MANAJEMEN PERANGKAT & PEMETAAN -->
        <div class="card">
          <div class="card-title">1. Manajemen Perangkat & Pemetaan Pengguna (RBAC)</div>
          
          <div style="font-size:12px; color:#4a6585; margin-bottom:6px;">Daftar Perangkat yang Diizinkan untuk Akun Ini:</div>
          <table>
            <thead>
              <tr>
                <th>Device ID</th>
                <th>Pemilik</th>
                <th>Petak Lahan (PlotMeta)</th>
                <th>Interval</th>
              </tr>
            </thead>
            <tbody id="deviceTableBody"></tbody>
          </table>

          <div style="margin-top:12px; font-weight:600; font-size:12px; color:#2c5282;">Ubah Interval Sampling Perangkat:</div>
          <div class="form-grid" style="margin-top:6px;">
            <div class="field">
              <label>Pilih Perangkat</label>
              <select id="ctrlDevice"></select>
            </div>
            <div class="field">
              <label>Interval Baru (detik)</label>
              <input type="number" id="ctrlInterval" value="30" />
            </div>
          </div>

          <div class="btn-row">
            <button class="btn-primary" onclick="submitIntervalChange()">Simpan Interval Baru</button>
            <button id="btnShowAddDev" onclick="toggleAddDeviceForm()">+ Registrasi Perangkat Baru</button>
          </div>
          <div id="ctrlResult" class="info-box">Siap mengubah konfigurasi perangkat.</div>

          <div id="addDeviceBox" style="display:none; margin-top:10px; padding:10px; background:#f4f9fd; border-radius:5px; border:1px solid #cce0f2;">
            <div style="font-weight:700; margin-bottom:8px; font-size:12px; color:#234975;">Form Pendaftaran Perangkat (Khusus Admin):</div>
            <div class="form-grid">
              <div class="field"><label>Device ID</label><input id="newDevId" value="ESP32-GAMBUT-03" /></div>
              <div class="field"><label>Username Pemilik</label><input id="newOwner" value="petani_budi" /></div>
              <div class="field"><label>Plot ID</label><input id="newPlotId" value="PLOT-KALTENG-03" /></div>
              <div class="field"><label>Nama Lahan</label><input id="newPlotName" value="Petak Gambut Blok C" /></div>
              <div class="field"><label>Tanaman</label><input id="newPlant" value="nanas" /></div>
              <div class="field"><label>Luas (Ha)</label><input type="number" step="0.1" id="newArea" value="1.8" /></div>
            </div>
            <button class="btn-primary" style="width:100%; margin-top:6px;" onclick="registerNewDevice()">Simpan Perangkat</button>
          </div>
        </div>

        <!-- PANEL 2: UJI HAK AKSES RBAC & KEPEMILIKAN -->
        <div class="card">
          <div class="card-title">2. Uji Otorisasi Hak Akses & Kepemilikan Perangkat</div>
          
          <div class="form-grid">
            <div class="field">
              <label>Perangkat Target</label>
              <select id="rbacDevice"></select>
            </div>
            <div class="field">
              <label>Aksi yang Diminta</label>
              <select id="rbacAction">
                <option value="Kontrol">Kontrol (Ubah Interval)</option>
                <option value="Baca">Baca Data</option>
                <option value="Hapus">Hapus Perangkat</option>
              </select>
            </div>
          </div>

          <button class="btn-primary" style="width:100%;" onclick="testCurrentRbacAction()">Uji Validasi Hak Akses Akun Ini</button>
          <div id="rbacResult" class="info-box">Klik tombol untuk menguji hak akses akun aktif terhadap perangkat target.</div>

          <div style="margin-top:12px; font-size:11px; color:#527196; line-height:1.4;">
            * <strong>Admin</strong>: Berhak Baca, Kontrol semua alat, dan Hapus perangkat.<br/>
            * <strong>Operator</strong>: Berhak Kontrol hanya untuk alat miliknya sendiri. Ditolak jika mengotak-atik alat orang lain.<br/>
            * <strong>Viewer</strong>: Hanya berhak membaca data perangkat.
          </div>
        </div>
      </div>

      <div class="grid-2">
        <!-- PANEL 3: MONITOR KONEKSI & OFFLINE BUFFER -->
        <div class="card">
          <div class="card-title">3. Monitoring Jaringan Gateway & Offline Buffer</div>
          
          <div class="btn-row">
            <button class="btn-danger" onclick="toggleNetwork(false)">Simulasi Sinyal Putus (Offline)</button>
            <button class="btn-primary" onclick="toggleNetwork(true)">Pulihkan Sinyal & Kirim Buffer (Online)</button>
          </div>

          <div id="bufferStatus" class="info-box">Mode koneksi gateway saat ini: ONLINE</div>

          <div style="margin-top:10px; font-weight:600; font-size:12px; color:#2c5282;">Antrean Data Offline Buffer:</div>
          <table>
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

        <!-- PANEL 4: ATURAN ANOMALI EDGE & DATA MODUL 3 -->
        <div class="card">
          <div class="card-title">4. Edge Processing & Integrasi Modul 3 (ReadingSink)</div>
          
          <div class="btn-row">
            <button onclick="applyPreset('normal')">Preset Normal</button>
            <button onclick="applyPreset('suspect')">Preset Kering (Suspect)</button>
            <button onclick="applyPreset('bad')">Preset Rusak (Bad)</button>
            <button onclick="applyPreset('spam')">Uji Interval Cepat</button>
          </div>

          <div class="form-grid">
            <div class="field"><label>Device ID</label><select id="inDevice"></select></div>
            <div class="field"><label>Waktu (s)</label><input type="number" id="inTsSec" value="1000" /></div>
            <div class="field"><label>Moisture (%)</label><input type="number" step="0.1" id="inMoisture" value="68.5" /></div>
            <div class="field"><label>Suhu (&deg;C)</label><input type="number" step="0.1" id="inTemp" value="28.4" /></div>
            <div class="field"><label>pH Tanah</label><input type="number" step="0.1" id="inPh" value="4.5" /></div>
            <div class="field"><label>EC (mS/cm)</label><input type="number" step="0.1" id="inEc" value="1.2" /></div>
          </div>

          <button class="btn-primary" style="width:100%;" onclick="sendSensorData()">Kirim Data Sensor ke Gateway</button>
          <div id="sensorResult" class="info-box">Siap mengirimkan data sensor.</div>

          <div style="margin-top:10px; font-weight:600; font-size:12px; color:#2c5282;">Data Tervalidasi yang Mengalir ke Modul 3:</div>
          <table>
            <thead>
              <tr>
                <th>Waktu</th>
                <th>Perangkat</th>
                <th>Nilai Sensor</th>
                <th>Quality</th>
                <th>Buzzer</th>
              </tr>
            </thead>
            <tbody id="m3TableBody">
              <tr><td colspan="5">Belum ada data masuk ke Modul 3.</td></tr>
            </tbody>
          </table>

          <div style="margin-top:8px; font-size:11px; font-weight:700; color:#3b5a80;">OUTPUT KONTRAK JSON MODUL 3:</div>
          <pre id="m3JsonPreview">// Output JSON data terakhir akan tampil di sini</pre>
        </div>
      </div>

    </div>
  </div>

  <script>
    const API_KEY = "TubesPF-demo-key";
    let eventCounter = 1;
    let currentUser = null;
    let globalState = null;
    let lastAcceptedTsByDevice = {};

    function quickFillLogin(username, password) {
      document.getElementById('loginUser').value = username;
      document.getElementById('loginPass').value = password;
      doLogin();
    }

    async function doLogin() {
      const u = document.getElementById('loginUser').value.trim();
      const p = document.getElementById('loginPass').value.trim();
      const msg = document.getElementById('loginMsg');

      try {
        const res = await fetch('/api/auth/login', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ username: u, password: p })
        });
        const data = await res.json();
        msg.style.display = 'block';

        if (res.status === 200 && data.success) {
          currentUser = data.user;
          msg.className = 'info-box ok';
          msg.textContent = data.message;
          setTimeout(() => {
            document.getElementById('loginSection').style.display = 'none';
            document.getElementById('dashboardSection').style.display = 'block';
            document.getElementById('authProfileBox').style.display = 'flex';
            document.getElementById('currentUsername').textContent = currentUser.username;
            document.getElementById('currentUserRole').textContent = currentUser.role;
            loadState();
          }, 350);
        } else {
          msg.className = 'info-box err';
          msg.textContent = `[Login Gagal] ${data.message}`;
        }
      } catch (e) {
        msg.style.display = 'block';
        msg.className = 'info-box err';
        msg.textContent = `Terjadi kesalahan koneksi login: ${e}`;
      }
    }

    async function doLogout() {
      await fetch('/api/auth/logout', { method: 'POST' });
      currentUser = null;
      document.getElementById('loginSection').style.display = 'block';
      document.getElementById('dashboardSection').style.display = 'none';
      document.getElementById('authProfileBox').style.display = 'none';
      const msg = document.getElementById('loginMsg');
      msg.className = 'info-box';
      msg.textContent = 'Anda telah logout.';
      msg.style.display = 'block';
    }

    async function loadState() {
      if (!currentUser) return;
      const res = await fetch('/api/state');
      globalState = await res.json();

      const netBadge = document.getElementById('netBadge');
      if (globalState.network_online) {
        netBadge.textContent = 'ONLINE';
        netBadge.style.color = '#1e6333';
      } else {
        netBadge.textContent = 'OFFLINE';
        netBadge.style.color = '#9c2525';
      }

      const allDevices = globalState.devices || [];
      const visibleDevices = (currentUser.role === 'Admin')
        ? allDevices
        : (currentUser.role === 'Operator')
          ? allDevices.filter(d => d.owner === currentUser.username)
          : allDevices;

      document.getElementById('kpiDevices').textContent = `${visibleDevices.length} / ${allDevices.length}`;
      document.getElementById('kpiBuffer').textContent = globalState.offline_buffer.length;
      document.getElementById('kpiM3').textContent = globalState.m3_sink_records.length;

      const lastRecord = globalState.m3_sink_records[globalState.m3_sink_records.length - 1];
      const kpiBuzzer = document.getElementById('kpiBuzzer');
      if (lastRecord && lastRecord.buzzer_alarm_active) {
        kpiBuzzer.textContent = 'MENYALA (BAHAYA)';
        kpiBuzzer.style.color = '#9c2525';
      } else {
        kpiBuzzer.textContent = 'MATI (AMAN)';
        kpiBuzzer.style.color = '#1e6333';
      }

      const ctrlDev = document.getElementById('ctrlDevice');
      const rbacDev = document.getElementById('rbacDevice');
      const inDev = document.getElementById('inDevice');
      const prevCtrl = ctrlDev.value;
      const prevRbac = rbacDev.value;
      const prevIn = inDev.value;

      ctrlDev.innerHTML = '';
      rbacDev.innerHTML = '';
      inDev.innerHTML = '';

      allDevices.forEach(d => {
        rbacDev.innerHTML += `<option value="${d.id}">${d.id} (Pemilik: ${d.owner || 'None'})</option>`;
        inDev.innerHTML += `<option value="${d.id}">${d.id} (${d.sampling_interval_sec}s)</option>`;
      });

      visibleDevices.forEach(d => {
        ctrlDev.innerHTML += `<option value="${d.id}">${d.id} (${d.sampling_interval_sec}s)</option>`;
      });

      if (prevCtrl) ctrlDev.value = prevCtrl;
      if (prevRbac) rbacDev.value = prevRbac;
      if (prevIn) inDev.value = prevIn;

      const devBody = document.getElementById('deviceTableBody');
      if (visibleDevices.length === 0) {
        devBody.innerHTML = '<tr><td colspan="4">Tidak ada perangkat yang ditugaskan ke akun ini.</td></tr>';
      } else {
        devBody.innerHTML = visibleDevices.map(d => `
          <tr>
            <td><strong>${d.id}</strong></td>
            <td>${d.owner || '-'}</td>
            <td>${d.plot ? `${d.plot.plot_id} (${d.plot.plant})` : '-'}</td>
            <td>${d.sampling_interval_sec} detik</td>
          </tr>
        `).join('');
      }

      document.getElementById('btnShowAddDev').style.display = (currentUser.role === 'Admin') ? 'inline-block' : 'none';

      const bufBody = document.getElementById('bufferTableBody');
      if (globalState.offline_buffer.length === 0) {
        bufBody.innerHTML = '<tr><td colspan="4">Antrean buffer kosong.</td></tr>';
      } else {
        bufBody.innerHTML = globalState.offline_buffer.map(item => `
          <tr>
            <td>${item.raw.event_id}</td>
            <td>${item.raw.device_id}</td>
            <td>${item.raw.moisture_raw}% / ${item.raw.temperature_raw}&deg;C / pH ${item.raw.ph_raw} / ${item.raw.ec_raw}</td>
            <td>${item.reason}</td>
          </tr>
        `).join('');
      }

      const m3Body = document.getElementById('m3TableBody');
      if (globalState.m3_sink_records.length === 0) {
        m3Body.innerHTML = '<tr><td colspan="5">Belum ada data masuk ke Modul 3.</td></tr>';
      } else {
        m3Body.innerHTML = globalState.m3_sink_records.slice().reverse().map(env => {
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

    async function submitIntervalChange() {
      const devId = document.getElementById('ctrlDevice').value;
      const intervalSec = parseInt(document.getElementById('ctrlInterval').value, 10);
      if (!devId) {
        alert("Pilih perangkat terlebih dahulu.");
        return;
      }
      const payload = {
        username: currentUser.username,
        role: currentUser.role,
        aksi: "Kontrol",
        device_id: devId,
        new_interval_sec: intervalSec
      };
      const res = await fetch('/api/rbac/action', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'X-API-Key': API_KEY },
        body: JSON.stringify(payload)
      });
      const data = await res.json();
      const box = document.getElementById('ctrlResult');
      box.className = 'info-box ' + (data.success ? 'ok' : 'err');
      box.textContent = `[${res.status}] ${data.message}`;
      await loadState();
    }

    async function testCurrentRbacAction() {
      const devId = document.getElementById('rbacDevice').value;
      const aksi = document.getElementById('rbacAction').value;
      const payload = {
        username: currentUser.username,
        role: currentUser.role,
        aksi: aksi,
        device_id: devId,
        new_interval_sec: 45
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

    function toggleAddDeviceForm() {
      const el = document.getElementById('addDeviceBox');
      el.style.display = (el.style.display === 'none') ? 'block' : 'none';
    }

    async function registerNewDevice() {
      if (currentUser.role !== 'Admin') {
        alert("Hanya Admin yang berhak mendaftarkan perangkat baru.");
        return;
      }
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
      const box = document.getElementById('ctrlResult');
      box.className = 'info-box ' + (data.success ? 'ok' : 'err');
      box.textContent = data.message;
      if (data.success) document.getElementById('addDeviceBox').style.display = 'none';
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
  </script>
</body>
</html>
"##;
