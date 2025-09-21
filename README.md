# Travaasa

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)  
[Live Demo](https://travaasa.onrender.com)

_Travaasa_ is a full-stack web application — my first end-to-end project. It’s built using Node.js, Express, EJS views, with a MongoDB-style schema setup. The goal is to learn and implement both front-end rendering and back-end logic in a cohesive app.

---

## Table of Contents

- [Features](#features)  
- [Tech Stack](#tech-stack)  
- [Getting Started](#getting-started)  
- [Project Structure](#project-structure)  
- [Usage](#usage)  
- [Contributing](#contributing)  
- [License](#license)  

---

## Features

- Rendered views using **EJS** templates  
- Routing with Express to handle various endpoints  
- Model layer for data schema definitions  
- Utilities and middleware to manage common tasks (authentication, request parsing, error handling, etc.)  
- Public assets served (CSS / images / front-end)  
- Deployed version live on Render  

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Backend   | Node.js, Express.js |
| Templating | EJS |
| Schema / Models | Custom schema setup (likely using Mongoose or equivalent) |
| Middleware | Custom middleware module |
| Routing | Express routers |
| Views / Frontend | EJS + CSS |
| Deployment | Render |

---

## Getting Started

### Prerequisites

- Node.js (v14 or higher recommended)  
- npm (Node package manager)  
- Access to MongoDB (local or cloud) if you’re using a database backend  

### Installation

1. Clone the repository  
   ```bash
   git clone https://github.com/ADIBAINS/travaasa.git
   cd travaasa
   ```

2. Install dependencies  
   ```bash
   npm install
   ```

3. Set up configuration / environment variables  
   You might need a `.env` file (if you haven’t already) with things like:  
   ```
   PORT=3000
   SECRET=some_secret
   CLOUD_NAME=sdvdsvfvf
   CLOUD_API_KEY=r3yg34g54hgh
   CLOUD_API_SECRET=idkwhatisit
   MAPBOX_TOKEN=getyourowntoken
   ATLASDB_URL=useandconfigit
   ```
   (Adjust according to your `cloudConfig.js` or wherever config is stored.)

4. Run the application  
   ```bash
   npm start
   ```

5. Open in browser  
   Visit `http://localhost:3000` (or whatever port you’ve configured)

---

## Project Structure

Here’s a breakdown of the important folders/files:

```
travaasa/
│
├── controllers/         # Request handlers, logic controlling routes
├── models/              # Data schema definitions
├── routes/              # Route definitions / endpoints
├── views/               # EJS templates for rendering pages
├── public/              # Static files (CSS, images, js)
├── util/                # Utilities/helpers used across project
├── middleware.js        # Custom middleware definitions
├── app.js               # Main entry point, express app setup
├── cloudConfig.js       # Configuration for cloud / deployment (e.g., environment variables)
├── schema.js            # Global schema definitions
├── package.json         # Dependencies, scripts
└── .gitignore           # Ignored files/folders
```

---

## Usage

- Register / log in (if applicable)  
- Navigate through the pages (depending on which routes are implemented)  
- If there is CRUD functionality — create, read, update, delete items  
- Make sure to test edge cases and invalid input  

---

## Contributing

Contributions are welcome! If you’d like to help:

1. Fork the repo  
2. Create a new branch (`git checkout -b feature/YourFeature`)  
3. Make your changes / additions  
4. Test your code  
5. Submit a pull request  

Please adhere to clean code practices, comment where needed, and follow consistent style.

---

## License

This project is licensed under the **MIT License** — see the [LICENSE](LICENSE) file for details.

---

## Future Improvements (Ideas)

- Improve styling / responsive front-end  
- Add unit / integration tests  
- Use more modular design or introduce front-end framework if needed  
- Transfrom into a mobile application  

---

**Thanks for checking out Travaasa!**

---

> *Built by Adil Bains*  
