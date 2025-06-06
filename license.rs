PK     ּ�Z��iS    
   backend.js
const express = require('express');
const cors = require('cors');
const path = require('path');
const db = require('./db');
const app = express();

app.use(cors());
app.use(express.json());
app.use(express.static(path.join(__dirname, 'public')));

// Cadastrar usuário
app.post('/usuarios', (req, res) => {
  const { login, senha } = req.body;
  const sql = 'INSERT INTO usuarios (login, senha) VALUES (?, ?)';
  db.query(sql, [login, senha], (err) => {
    if (err) {
      if (err.code === 'ER_DUP_ENTRY') {
        return res.status(400).send('Usuário já existe!');
      }
      return res.status(500).send(err);
    }
    res.send('Usuário cadastrado com sucesso!');
  });
});

// Login
app.post('/login', (req, res) => {
  const { login, senha } = req.body;
  const sql = 'SELECT * FROM usuarios WHERE login = ? AND senha = ?';
  db.query(sql, [login, senha], (err, results) => {
    if (err) return res.status(500).send(err);
    if (results.length > 0) res.send('Login válido');
    else res.status(401).send('Login ou senha inválidos!');
  });
});

// Obter ordens por usuário
app.get('/ordens/:usuario', (req, res) => {
  const sql = 'SELECT * FROM ordens WHERE usuario = ?';
  db.query(sql, [req.params.usuario], (err, results) => {
    if (err) return res.status(500).send(err);
    res.json(results);
  });
});

// Criar nova ordem
app.post('/ordens', (req, res) => {
  const { tipo, inicio, conclusao, usuario } = req.body;
  const sql = 'INSERT INTO ordens (tipo, inicio, conclusao, usuario) VALUES (?, ?, ?, ?)';
  db.query(sql, [tipo, inicio, conclusao, usuario], (err) => {
    if (err) return res.status(500).send(err);
    res.send('Ordem criada com sucesso!');
  });
});

// Excluir ordem
app.delete('/ordens/:id', (req, res) => {
  const sql = 'DELETE FROM ordens WHERE id = ?';
  db.query(sql, [req.params.id], (err) => {
    if (err) return res.status(500).send(err);
    res.send('Ordem excluída!');
  });
});

const PORT = 3000;
app.listen(PORT, () => console.log(`Servidor rodando em http://localhost:${PORT}`));
PK     ּ�Z��;��  �     db.js
const mysql = require('mysql2');

const connection = mysql.createConnection({
  host: '127.0.0.1',
  user: 'gestor',
  password: '',
  database: 'sistema_gestao',
  port: 3306
});

connection.connect(err => {
  if (err) {
    console.error('Erro ao conectar ao MySQL:', err);
    process.exit();
  } else {
    console.log('Conectado ao MySQL!');
  }
});

module.exports = connection;
PK     ּ�Z��9�?  ?     public/index.html
<!DOCTYPE html>
<html lang="pt-BR">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0"/>
  <title>Sistema de Gestão</title>
  <style>
    body {
      font-family: Arial, sans-serif;
      margin: 0;
      background-color: #f0f2f5;
    }
    .container {
      background-color: #fff;
      max-width: 600px;
      margin: 40px auto;
      padding: 20px 30px;
      border-radius: 10px;
      box-shadow: 0 0 15px rgba(0,0,0,0.2);
    }
    h2 {
      text-align: center;
      color: #333;
      border-bottom: 2px solid #007bff;
      padding-bottom: 5px;
    }
    form, .ordens-container {
      margin-top: 20px;
    }
    input, button {
      width: 100%;
      padding: 10px;
      margin-top: 10px;
      border-radius: 6px;
      border: 1px solid #ccc;
      font-size: 16px;
    }
    button {
      background-color: #007bff;
      color: white;
      border: none;
      font-weight: bold;
      cursor: pointer;
    }
    button:hover {
      background-color: #0056b3;
    }
    .hidden { display: none; }
    .ordem {
      padding: 10px;
      background: #f5f5f5;
      border: 1px solid #ccc;
      border-radius: 6px;
      margin-bottom: 10px;
    }
    .ordem button {
      width: auto;
      margin-right: 5px;
    }
  </style>
</head>
<body>
  <div class="container">
    <form id="cadastroForm">
      <h2>Cadastro</h2>
      <input type="text" id="novoLogin" placeholder="Login" required />
      <input type="password" id="novaSenha" placeholder="Senha" required />
      <input type="password" id="confirmarSenha" placeholder="Confirmar Senha" required />
      <button type="submit">Cadastrar</button>
    </form>

    <form id="loginForm" class="hidden">
      <h2>Login</h2>
      <input type="text" id="login" placeholder="Login" required />
      <input type="password" id="senha" placeholder="Senha" required />
      <button type="submit">Entrar</button>
    </form>

    <form id="ordemServicoForm" class="hidden">
      <h2>Nova Ordem de Serviço</h2>
      <input type="text" id="tipo" placeholder="Tipo de Serviço" required />
      <input type="date" id="dataInicio" required />
      <input type="date" id="dataConclusao" required />
      <button type="submit">Salvar Ordem</button>
    </form>

    <div class="ordens-container hidden" id="ordensContainer">
      <h2>Ordens de Serviço</h2>
      <div id="listaOrdens"></div>
      <button onclick="mostrarFormulario('ordemServico')">Criar Nova Ordem</button>
      <button onclick="logout()">Logout</button>
    </div>
  </div>

  <script>
    let usuarioLogado = null;

    function mostrarFormulario(id) {
      document.querySelectorAll('form, .ordens-container').forEach(e => e.classList.add('hidden'));
      document.getElementById(id + 'Form')?.classList.remove('hidden');
      if (id === 'ordens') document.getElementById('ordensContainer').classList.remove('hidden');
    }

    async function atualizarListaOrdens() {
      const res = await fetch(`http://localhost:3000/ordens/${usuarioLogado}`);
      const ordens = await res.json();
      const lista = document.getElementById('listaOrdens');
      lista.innerHTML = '';
      ordens.forEach(ordem => {
        const div = document.createElement('div');
        div.className = 'ordem';
        div.innerHTML = `
          <strong>Tipo:</strong> ${ordem.tipo}<br>
          <strong>Início:</strong> ${ordem.inicio}<br>
          <strong>Conclusão:</strong> ${ordem.conclusao}<br>
          <button onclick="editarOrdem(${ordem.id}, '${ordem.tipo}', '${ordem.inicio}', '${ordem.conclusao}')">Editar</button>
          <button onclick="excluirOrdem(${ordem.id})">Excluir</button>
        `;
        lista.appendChild(div);
      });
    }

    function editarOrdem(id, tipo, inicio, conclusao) {
      document.getElementById('tipo').value = tipo;
      document.getElementById('dataInicio').value = inicio;
      document.getElementById('dataConclusao').value = conclusao;
      excluirOrdem(id);
      mostrarFormulario('ordemServico');
    }

    async function excluirOrdem(id) {
      if (confirm('Deseja excluir esta ordem?')) {
        await fetch(`http://localhost:3000/ordens/${id}`, { method: 'DELETE' });
        atualizarListaOrdens();
      }
    }

    function logout() {
      usuarioLogado = null;
      mostrarFormulario('login');
    }

    document.addEventListener('DOMContentLoaded', () => {
      mostrarFormulario('cadastro');

      document.getElementById('cadastroForm').addEventListener('submit', async e => {
        e.preventDefault();
        const login = document.getElementById('novoLogin').value;
        const senha = document.getElementById('novaSenha').value;
        const confirmar = document.getElementById('confirmarSenha').value;

        if (senha !== confirmar) return alert('As senhas não coincidem!');

        try {
          const res = await fetch('http://localhost:3000/usuarios', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ login, senha })
          });
          if (!res.ok) throw new Error(await res.text());
          alert('Usuário cadastrado com sucesso!');
          mostrarFormulario('login');
        } catch (err) {
          alert(err.message);
        }
      });

      document.getElementById('loginForm').addEventListener('submit', async e => {
        e.preventDefault();
        const login = document.getElementById('login').value;
        const senha = document.getElementById('senha').value;

        try {
          const res = await fetch('http://localhost:3000/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ login, senha })
          });

          if (!res.ok) throw new Error(await res.text());

          usuarioLogado = login;
          alert('Login realizado com sucesso!');
          mostrarFormulario('ordens');
          atualizarListaOrdens();
        } catch (err) {
          alert(err.message);
        }
      });

      document.getElementById('ordemServicoForm').addEventListener('submit', async e => {
        e.preventDefault();
        const tipo = document.getElementById('tipo').value;
        const inicio = document.getElementById('dataInicio').value;
        const conclusao = document.getElementById('dataConclusao').value;

        if (new Date(inicio) > new Date(conclusao)) {
          return alert('A data de início deve ser anterior à data de conclusão.');
        }

        try {
          const res = await fetch('http://localhost:3000/ordens', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ tipo, inicio, conclusao, usuario: usuarioLogado })
          });
          if (!res.ok) throw new Error(await res.text());
          alert('Ordem salva com sucesso!');
          atualizarListaOrdens();
          mostrarFormulario('ordens');
          e.target.reset();
        } catch (err) {
          alert(err.message);
        }
      });
    });

    window.editarOrdem = editarOrdem;
    window.excluirOrdem = excluirOrdem;
  </script>
</body>
</html>
PK     ּ�Z��iS    
           ��    backend.jsPK     ּ�Z��;��  �             ��)  db.jsPK     ּ�Z��9�?  ?             ���	  public/index.htmlPK      �   <&    