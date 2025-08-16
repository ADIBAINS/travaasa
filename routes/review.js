const express = require('express');
const router = express.Router({mergeParams : true});
const wrapAsync = require('../util/wrapAsync.js');
const Review = require("../models/review.js");
const Listing = require("../models/listing.js");
const {validateReview, isLoggedIn, isReviewAuthor} = require('../middleware.js');
const reviewController = require('../controllers/reviews.js');


// POST REVIEW ROUTE
router.post("/",isLoggedIn, validateReview, wrapAsync(reviewController.createReview));

// DELETE REVIEW ROUTE
router.delete("/:reviewid",isLoggedIn,isReviewAuthor, wrapAsync(reviewController.destroyReview));


module.exports = router;