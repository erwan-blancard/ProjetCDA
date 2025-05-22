function login_submit() {
    const username = document.getElementById("username-input").value;
    const password = document.getElementById("password-input").value;
    const payload = { username, password };
    const request = new XMLHttpRequest();
    request.onreadystatechange = () => {
      if (request.readyState === XMLHttpRequest.DONE) {
        if (request.status === 200) {
          window.location.href = "/index.html";
        }
      }
    };
    request.open("POST", "http://localhost:8080/login");
    request.setRequestHeader("Content-Type", "application/json");
    request.withCredentials = true;
    request.send(JSON.stringify(payload));
  }
  
  function register_submit() {
    const username = document.getElementById("username-input").value;
    const email = document.getElementById("email-input").value;
    const password = document.getElementById("password-input").value;
    const payload = { username, email, password };
    const request = new XMLHttpRequest();
    request.onreadystatechange = () => {
      if (request.readyState === XMLHttpRequest.DONE) {
        if (request.status === 200) {
          window.location.href = "/login.html";
        }
      }
    };
    request.open("POST", "http://localhost:8080/register");
    request.setRequestHeader("Content-Type", "application/json");
    request.send(JSON.stringify(payload));
  }
  
  function logout() {
    document.cookie = "token=";
    window.location.href = "/login.html";
  }
  
  function getCookie(name) {
    const match = document.cookie.match(new RegExp("(^| )" + name + "=([^;]+)"));
    if (match) return match[2];
    return null;
  }
  
  export { login_submit, register_submit, logout, getCookie };
/**
 * @jest-environment jsdom
 */
import { login_submit, register_submit, logout, getCookie } from "../auth";

describe("auth.js", () => {
  let realXHR;
  beforeAll(() => {
    realXHR = window.XMLHttpRequest;
  });

  afterAll(() => {
    window.XMLHttpRequest = realXHR;
  });

  test("login_submit envoie un POST et redirige si 200", () => {
    document.body.innerHTML = `
      <input id="username-input" value="user">
      <input id="password-input" value="pass">
    `;
    const calls = [];
    class FakeXHR {
      constructor() {
        this.readyState = 0;
        this.status = 0;
        this.headers = {};
        calls.push({ ctor: true });
      }
      open(method, url) {
        calls.push({ open: [method, url] });
      }
      setRequestHeader(k, v) {
        calls.push({ header: [k, v] });
      }
      send(body) {
        this.readyState = XMLHttpRequest.DONE;
        this.status = 200;
        calls.push({ send: body });
        this.onreadystatechange();
      }
    }
    window.XMLHttpRequest = FakeXHR;
    login_submit();
    expect(calls).toEqual([
      { ctor: true },
      { open: ["POST", "http://localhost:8080/login"] },
      { header: ["Content-Type", "application/json"] },
      { send: JSON.stringify({ username: "user", password: "pass" }) }
    ]);
    expect(window.location.href).toContain("/index.html");
  });

  test("register_submit envoie un POST et redirige si 200", () => {
    document.body.innerHTML = `
      <input id="username-input" value="u">
      <input id="email-input" value="e@mail">
      <input id="password-input" value="p">
    `;
    const calls = [];
    class FakeXHR {
      constructor() { calls.push({ ctor: true }); }
      open(m, u) { calls.push({ open: [m, u] }); }
      setRequestHeader(k, v) { calls.push({ header: [k, v] }); }
      send(b) {
        calls.push({ send: b });
        this.readyState = XMLHttpRequest.DONE;
        this.status = 200;
        this.onreadystatechange();
      }
    }
    window.XMLHttpRequest = FakeXHR;
    register_submit();
    expect(calls).toEqual([
      { ctor: true },
      { open: ["POST", "http://localhost:8080/register"] },
      { header: ["Content-Type", "application/json"] },
      { send: JSON.stringify({ username: "u", email: "e@mail", password: "p" }) }
    ]);
    expect(window.location.href).toContain("/login.html");
  });

  test("logout supprime le cookie et redirige", () => {
    document.cookie = "token=abc";
    logout();
    expect(document.cookie).toBe("token=");
    expect(window.location.href).toContain("/login.html");
  });

  test("getCookie récupère la valeur existante ou null", () => {
    document.cookie = "a=1; b=2";
    expect(getCookie("a")).toBe("1");
    expect(getCookie("b")).toBe("2");
    expect(getCookie("nope")).toBeNull();
  });
});
  