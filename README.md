# How the APIs work for this web app

For the cycle of each request for a page:

1. Initially load the page with static elements, which are those that do not require a query to the database.
2. Using HTMX, the initial markup will include htmx attributes for the container if they
require to have content in them that's dynamic. Their initial content will be skeleton loaders.
