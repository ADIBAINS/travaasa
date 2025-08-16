const Listing = require("./models/listing");
const Review = require("./models/review.js");
const {reviewSchema,listingSchema } = require('./schema.js');
const ExpressError = require("./util/ExpressError.js");

module.exports.isLoggedIn = (req,res,next)=>{
    if(!req.isAuthenticated()){
        req.session.redirectUrl = req.originalUrl;
        req.flash("error","you must be logged in to create listings!");
        return res.redirect("/login");
    }
    next();

};
module.exports.saveRedirectUrl = (req,res,next) =>{
    if (req.session.redirectUrl){
        res.locals.redirectUrl = req.session.redirectUrl;
    }
    next();
};


module.exports.isOwner = async(req,res,next) =>{
    const { id } = req.params;
    let listing = await Listing.findById(id);
    if(!listing.owner._id.equals(res.locals.userlog._id)){
        req.flash("error", "You don't have the access to edit");
        return res.redirect(`/listings/${id}`);
    }
    next();
};

module.exports.validateListing = ((req, res, next) => {
    let { error } = listingSchema.validate(req.body);
    if (error) {
        throw new ExpressError(400, error);
    } else {
        next();
    }
});

module.exports.validateReview = ((req, res, next) => {
    let { error } = reviewSchema.validate(req.body);
    if (error) {
        throw new ExpressError(400, error);
    } else {
        next();
    }
});

module.exports.isReviewAuthor = async(req,res,next) =>{
    const { id, reviewid } = req.params;
    let review = await Review.findById(reviewid);
    if(!review.author.equals(res.locals.userlog._id)){
        req.flash("error", "You don't have the access to edit");
        return res.redirect(`/listings/${id}`);
    }
    next();
};