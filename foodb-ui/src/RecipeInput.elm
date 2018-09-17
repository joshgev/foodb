module RecipeInput exposing (makeSelect)
import Html exposing (..)
import Html.Events as HE
import Html.Attributes as HA

import Json.Decode as D
import Json.Encode as E



--------------------------- SELECT ------------------------

toOption : (Maybe a -> m) -> (a -> E.Value) -> (a -> Html m) -> a -> Html m
toOption tagger encoder renderer x = 
    option [HA.value (E.encode 0 (encoder x))] [renderer x] 


--categoryFromString : String -> Maybe RT.Category
--categoryFromString string =
--    case Debug.log "arg" (Decode.decodeString RJ.categoryDecoder string) of 
--        Err _ -> Nothing
--        Ok x -> case x.category_id of
--            Nothing -> Nothing
--            Just _ -> Just x

fromString : D.Decoder a -> String -> Maybe a
fromString decoder string = 
    -- Note we are adding nullable to the decoder
    case (D.decodeString (D.nullable decoder) string) of
        Err _ -> Nothing
        Ok x -> case x of
            Nothing -> Nothing
            Just y -> Just y

onSelectChange : D.Decoder a -> (Maybe a -> m) -> Html.Attribute m
onSelectChange decoder tagger = 
    HE.on
    "change"
    (D.map
        tagger
        (D.map (fromString decoder) HE.targetValue))

toOption2 : (Maybe a -> m) -> (a -> E.Value ) -> (a -> Html m) -> Maybe a -> Html m
toOption2 tagger encoder renderer m =
    case m of 
        Nothing -> option [HA.value "null"] [text "--"]
        Just x -> toOption tagger encoder renderer x

renderSelectOptions : (Maybe a -> m) -> (a -> E.Value) -> (a -> Html m) -> List a -> List (Html m)
renderSelectOptions tagger encoder renderer categories = 
    List.map
        (toOption2 tagger encoder renderer)
        (List.append [Nothing] (List.map (\ x -> Just x) categories))
        

makeSelect : (a -> E.Value)  -> D.Decoder a -> (Maybe a -> m) -> (a -> Html m) -> List a -> Html m
makeSelect encoder decoder tagger renderer list = 
    select [ onSelectChange decoder tagger ] (renderSelectOptions tagger encoder renderer list)