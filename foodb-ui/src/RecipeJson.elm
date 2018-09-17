module RecipeJson exposing 
    (
        categoryDecoder
        ,categoriesDecoder
        ,categoryEncoder
        ,verbDecoder
        ,verbsDecoder
        ,verbEncoder)


import Json.Decode as D
import Json.Encode as E
import RecipeTypes as RT

------------------------------ General -------------------------
idDecoder : D.Decoder Int
idDecoder = D.field "id" D.int

nameDecoder : D.Decoder String
nameDecoder = D.field "name" D.string

------------------------------ CATEGORY -------------------------

conditionalIdEncoder : RT.Category -> E.Value
conditionalIdEncoder category =
    case category.category_id of
        Nothing -> E.null
        Just i -> E.int i

categoryEncoder: RT.Category -> E.Value
categoryEncoder category = E.object 
    [("name", E.string category.name)
    ,("category_id", conditionalIdEncoder category)]

categoryDecoder: D.Decoder RT.Category
categoryDecoder = D.map2 RT.Category (D.field "category_id" (D.nullable D.int)) nameDecoder

categoriesDecoder : D.Decoder (List RT.Category)
categoriesDecoder = D.list categoryDecoder


------------------------------ VERB -------------------------
verbEncoder : RT.Verb -> E.Value
verbEncoder verb = E.object [("verb", E.string verb.verb)]

verbDecoder : D.Decoder String
verbDecoder = D.field "verb" D.string

verbsDecoder : D.Decoder (List String)
verbsDecoder = D.list verbDecoder