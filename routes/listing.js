const express = require('express');
const router = express.Router();
const wrapAsync = require('../util/wrapAsync.js');
const Listing = require("../models/listing.js");
const { isLoggedIn, isOwner,validateListing} = require('../middleware.js');
const listingController = require('../controllers/listings.js');
const multer = require('multer');
const {storage, cloudinary} = require("../cloudConfig.js");
const upload = multer({ storage });


router.route("/")
.get(wrapAsync(listingController.index))
.post(isLoggedIn, validateListing,upload.single("listing[image]") ,wrapAsync(listingController.uploadNewListing));

// NEW ROUTE
router.get("/new",isLoggedIn,listingController.renderNewForm);

router.route("/:id")
.get(wrapAsync(listingController.showListing))
.put(isLoggedIn,isOwner, validateListing,upload.single("listing[image]") ,wrapAsync(listingController.uploadEditData))
.delete(isLoggedIn,isOwner, wrapAsync(listingController.destroyListing));

// EDIT ROUTE
router.get('/:id/edit',isLoggedIn,isOwner, wrapAsync(listingController.renderEditForm));



module.exports = router;