import Browser
import Html exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as Decode
import Url.Builder as Url
import Debug
import Basics

main = Browser.element
    { init=init, update=update, subscriptions=subscriptions, view=view}

type alias Model = {categories: List String, message: String}

init: () -> (Model, Cmd Msg)
init _ = 
    ((Model []) "nah", getCategories) 

type Msg = GetCategories | Categories (Result Http.Error (List String))

update : Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of
        GetCategories -> (model, Cmd.none)
        Categories result -> 
            case result of 
                Ok categories -> ({ model | categories = categories, message="yeah"}, Cmd.none)
                Err e -> ({model | message = (Debug.log "a" (Debug.toString  e))}, Cmd.none)

subscriptions: Model -> Sub Msg
subscriptions model = Sub.none

view : Model -> Html Msg
view model = 
    div [] [
        text model.message,
        ul [] (List.map toLi model.categories)
    ]
toLi : String -> Html msg
toLi x = li [] [text x]

getCategories : Cmd Msg
getCategories = Http.send Categories (Http.get "http://localhost:8001/categories" categoriesDecoder)

categoriesDecoder : Decode.Decoder (List String)
categoriesDecoder = Decode.list (Decode.field "name" Decode.string)