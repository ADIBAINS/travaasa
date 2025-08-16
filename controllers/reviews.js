const Review = require('../models/review');
const Listing = require("../models/listing.js");


module.exports.createReview = async (req, res) => {
    let listing = await Listing.findById(req.params.id);
    let newReview = new Review(req.body.review);
    newReview.author = req.user._id;
    listing.review.push(newReview);
    await listing.save();
    await newReview.save();
    req.flash("success","Review Created Successfully!");
    res.redirect(`/listings/${req.params.id}`);
};

module.exports.destroyReview = async (req, res) => {
    const { id, reviewid } = req.params;
    await Listing.findByIdAndUpdate(id, { $pull: { review: reviewid } });
    await Review.findByIdAndDelete(reviewid);
    req.flash("success","Review Deleted Successfully!");
    res.redirect(`/listings/${id}`);

};