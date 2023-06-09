let current_time = Math.floor(+(new Date())/1000);
let period = 60*60*24;

let a=getStartOfDayTimestamp();
let b=current_time;

function getStartOfDayTimestamp() {
  const now = new Date();
  const year = now.getFullYear();
  const month = now.getMonth();
  const date = now.getDate();

  const sod = new Date(year, month, date, 0, 0, 0);
  return Math.floor(sod.getTime() / 1000);
}

function changegraph(time) {
  time += 60*60*24;
  a=time - period;
  b=time;
  clearInterval(interval);
  get_timeline();
  get_sankey();
  get_stats();
  get_band();
}

async function getData() {

  switch (document.getElementById("period").value) {
    case "today":
      let current_time = Math.floor(+(new Date())/1000);
      a=getStartOfDayTimestamp();
      b=current_time;
      clearInterval(interval);
      get_timeline();
      get_sankey();
      get_stats();
      get_band();
      return;
    case "1-hour":
      period = 60*60;
      break;
    case "6-hours":
      period = 60*60*6;
      break;
    case "12-hours":
      period = 60*60*12;
      break;
    case "24-hours":
      period = 60*60*24;
      break;
    case "weekly":
      period = 60*60*24*7;
      break;
    case "monthly":
      period = 60*60*24*30;
      break;
    case "yearly":
      period = 60*60*24*365;
      break;
    default:
      break;
  }

  a=current_time - period;
  b=current_time;

  clearInterval(interval);
  get_timeline();
  get_sankey();
  get_stats();
  get_band();
}

async function get_timeline() {
  const response = await fetch("/timeline?width=" + "1920" + "&height=" + "80");
  const text = await response.text();
  document.getElementById("timeline").innerHTML = text;

  apply_hover();
}

async function get_stats() {
  let start_time = a;
  let end_time = b;

  const response = await fetch("/stats?start_time=" + start_time + "&end_time=" + end_time);
  const text = await response.text();
  document.getElementById("stats").innerHTML = text;
}

async function get_sankey() {
  let start_time = a;
  let end_time = b;

  const response = await fetch("/sankey?start_time=" + start_time + "&end_time=" + end_time + "&width=" + window.innerWidth + "&height=" + window.innerHeight);
  const text = await response.text();
  document.getElementById("sankey").innerHTML = text;
  document.getElementById("sankey").setAttribute("style",`height:${window.innerHeight}px`);

  d3.selectAll("text").each(function() {
    this.parentNode.appendChild(this);
  });

  apply_hover();
}

async function get_band() {
  let start_time = a;
  let end_time = b;

  const response = await fetch("/band?start_time=" + start_time + "&end_time=" + end_time + "&width=" + window.innerWidth + "&height=" + window.innerHeight);
  const text = await response.text();
  document.getElementById("band").innerHTML = text;
  document.getElementById("band").setAttribute("style",`height:${window.innerHeight}px`);

  apply_hover();
}

function apply_hover() {
  const hoverElements = document.querySelectorAll('.hover-element');

  const tooltip = document.getElementById('tooltip');

  hoverElements.forEach((element) => {
    element.addEventListener('mouseover', (event) => {
      element.classList.add('hover-highlight');
      tooltip.innerHTML = element.getAttribute('data-tooltip');
      tooltip.style.display = 'block';
    });

    element.addEventListener('mouseout', () => {
      element.classList.remove('hover-highlight');
      tooltip.style.display = 'none';
    });

    element.addEventListener('mousemove', (event) => {
      tooltip.style.top = `${event.pageY + 10}px`;

      if (event.pageX > window.innerWidth - 100) {
        tooltip.style.left = `${event.pageX - 100}px`;
      }else {
        tooltip.style.left = `${event.pageX + 10}px`;
      }
    });
  });
}

get_timeline();
get_sankey();
get_stats();
get_band();
let interval = setInterval(function() {
  a = getStartOfDayTimestamp();
  b = Math.floor(+(new Date())/1000);
  get_sankey();
  get_stats();
  get_band();
}, 10000);
