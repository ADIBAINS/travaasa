const User = require('../models/user');

module.exports.signupForm = (req,res) =>{
    res.render("users/signup.ejs");
};

module.exports.createUser = async(req,res)=>{   
    try{ 
    let {username, email, password} = req.body;
    const newUser = new User({email,username});
    const registerdUser = await User.register(newUser,password);
    req.login(registerdUser,(err)=>{
        if(err){
            return next(err);
        }
        req.flash("success", "Welcome to TRAVAASA");
        res.redirect("/listings");
    })
    }catch (e){
        req.flash('error',e.message);
        res.redirect("/signup");
    }
};

module.exports.loginForm = (req,res)=>{
    res.render("users/login.ejs");
};

module.exports.loginUser = async (req,res) =>{
    let redirectUrl = res.locals.redirectUrl || "/listings";
    res.redirect(redirectUrl);
};

module.exports.logout = (req,res,next)=>{
    req.logOut((err)=>{
        if (err){
           return next(err);
        }
        req.flash("success","Successful Logout");
        res.redirect("/listings");
    })
};