document.body.style.cursor = "default";

window.onload = function() {
  setTimeout(function() {
    const resizer = document.querySelector(".resizer");
    const sidebar = document.querySelector(".sidebars");
    

    resizer.addEventListener("mousedown", (event) => {
      document.body.style.cursor = "col-resize";
      document.addEventListener("mousemove", resize, false);

      document.addEventListener("mouseup", () => {
        document.removeEventListener("mousemove", resize, false);
        document.body.style.cursor = 'default';
      }, false);
    });

    function resize(e) {
      const size = `${e.x}px`;
      sidebar.style.width = size;
    }

  }, 1000);


  setTimeout(function() {
    const resizer2 = document.querySelector(".resizer2");
    
    resizer2.addEventListener("mousedown", (event) => {
      document.body.style.cursor = "row-resize";
      document.addEventListener("mousemove", resize, false);

      document.addEventListener("mouseup", () => {
        document.removeEventListener("mousemove", resize, false);
        document.body.style.cursor = 'default';
      }, false);
    });

    function resize(e) {
    const req = document.querySelector(".req");
    const resp = document.querySelector(".resp");
    const reqbody = document.querySelector(".reqbody");
    const respbody = document.querySelector(".respbody");
    const reqheaders = document.querySelector(".reqheaders");      
      
      const req_size = e.y;
      req.style.height = `${req_size}px`;

      try {
        let req_body_size = req.clientHeight - 140;
        reqbody.style.height = `${req_body_size}px`;
      } catch(err) {
        let reqheaders_size = req.clientHeight - 115;
        reqheaders.style.height = `${reqheaders_size}px`;
      }

      let whole = document.documentElement.scrollHeight;
     
      let resp_size = whole - req_size - 90;

      resp.style.height = `${resp_size}px`;
      
    }

    init_resize();    
  }, 1000);
};


window.addEventListener("resize", init_resize);


function init_resize() {
  // console.log("init called");

  const sidebar = document.querySelector(".sidebars");
  const resp = document.querySelector(".resp");
  const url_input = document.querySelector(".urlinput");
  const req = document.querySelector(".req");
  const reqheaders = document.querySelector(".reqheaders");
  const reqbody = document.querySelector(".reqbody");
  
  url_input.focus();
  
  sidebar.style.width = '250px';
  
  req.style.height = '325px';


  try {
      let req_body_size = req.clientHeight - 140;
      reqbody.style.height = `${req_body_size}px`;
  } catch(err) {
    let reqheaders_size = req.clientHeight - 115;
    reqheaders.style.height = `${reqheaders_size}px`;
  }

  let whole = document.documentElement.scrollHeight;

  let req_size = req.clientHeight;

  let resp_size = whole - req_size - 90;

  resp.style.height = `${resp_size}px`;
}