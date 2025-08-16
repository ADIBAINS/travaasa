const express = require('express');
const router = express.Router();
const User = require('../models/user');
const wrapAsync = require('../util/wrapAsync');
const passport = require('passport');
const { saveRedirectUrl } = require('../middleware');
const userController = require('../controllers/users');


router.route("/signup")
.get(userController.signupForm)
.post(wrapAsync(userController.createUser));

router.route("/login")
.get(userController.loginForm)
.post(saveRedirectUrl, passport.authenticate("local",{failureRedirect : "/login", failureFlash : true}),userController.loginUser);

router.get("/logout",userController.logout);

module.exports = router;