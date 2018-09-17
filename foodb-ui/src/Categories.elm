--module Categories exposing (Categories)

import Browser
import Html exposing (..)
import Html.Events exposing (..)
import Html.Attributes as Attributes
import Http
import Json.Decode as Decode
import Json.Encode as Encode
import Url.Builder as Url
import Debug
import Basics
import RecipeTypes as RT
import RecipeJson as RJ
import RecipeInput as RI

main = Browser.element
    { init=init, update=update, subscriptions=subscriptions, view=view}

type alias Model = {
    category: Maybe RT.Category,
    categories: List RT.Category, 
    newCategory: String,
    recipeName: String,
    verbs: List String,
    newVerb: String,
    recipe: RT.Recipe
    }

init: () -> (Model, Cmd Msg)
init _ = 
    (Model Nothing [] "" "" [] "" (RT.Recipe "" (RT.Category Nothing "test") []), genericGet "http://localhost:8001/categories" GotCategories RJ.categoriesDecoder) 

type ResultType = 
    CategoryResult (RT.Category) 
    | VerbResult (RT.Verb)

type Msg = 
    CreateCategory 
    | UpdateCategory (Maybe RT.Category)
    | GotAPIResult (Result Http.Error ResultType)
    | NewCategoryUpdate (String)
    | RecipeNameUpdate (String)
    | GetCategories 
    | GotCategory (Result Http.Error RT.Category)
    | GotCategories (Result Http.Error (List RT.Category))
    | CreateVerb
    | NewVerbUpdate (String)
    | GotVerb (Result Http.Error String)
    | GotVerbs (Result Http.Error (List String))
    | AddStep


update : Msg -> Model -> (Model, Cmd Msg)
update msg model = 
    case msg of
        GetCategories -> (model, Cmd.none)
        GotCategories result -> 
            case result of 
                Ok categories -> ({ model | categories = (Debug.log "categories" categories)}, Cmd.none)
                Err e -> ({model | recipeName = (Debug.log "fuck" (Debug.toString e))}, Cmd.none)
        CreateCategory -> 
            (
                model
                , genericPost 
                    "http://localhost:8001/categories"
                    (RT.Category Nothing model.newCategory)
                    GotCategory
                    RJ.categoryEncoder
                    RJ.categoryDecoder
            )
        GotCategory _ -> (model, genericGet "http://localhost:8001/categories" GotCategories RJ.categoriesDecoder)
        NewCategoryUpdate newCategory -> ({model | newCategory = newCategory}, Cmd.none)
        RecipeNameUpdate recipeName -> (model, Cmd.none)

        -- WIP
        CreateVerb -> (model, Cmd.none)
        NewVerbUpdate _ -> (model, Cmd.none)
        GotVerb _ -> (model, Cmd.none)
        GotVerbs _ -> (model, Cmd.none)
        GotAPIResult _ -> (model, Cmd.none)
        AddStep -> (model, Cmd.none)
        UpdateCategory category -> ({ model | category = Debug.log "x" category}, Cmd.none)



subscriptions: Model -> Sub Msg
subscriptions model = Sub.none

view : Model -> Html Msg
view model = 
    div [] [
        h1 [] [text "New Recipe"],
        div [Attributes.style "display" "inline-block"] [
            text "Name",
            input [Attributes.placeholder "Name", Attributes.value model.recipeName, onInput RecipeNameUpdate] []
        ],
        div [Attributes.style "display" "inline-block"] [
            div [] [text "Category"],
            --makeSelect model.categories,
            RI.makeSelect
                RJ.categoryEncoder
                RJ.categoryDecoder
                UpdateCategory
                (\x -> text x.name)
                model.categories,
            --genericSelect model.categories (\ x -> x.name) (\ x -> x.id) (\ x -> (RT.Category "fuck")),
            div [] [
                input [Attributes.placeholder "New Category", Attributes.value model.newCategory, onInput NewCategoryUpdate] [],
                button [onClick CreateCategory] [text "Add"]
            ]
        ],
        br [] [],
        button [onClick AddStep] [text "Add Step"],
        renderCategory model.category,
        stepsView model.recipe.steps
        
    ]

genericPost : String -> a -> (Result Http.Error a -> Msg) -> (a -> Encode.Value) -> Decode.Decoder a -> Cmd Msg 
genericPost url data m encoder decoder = 
    Http.send
        m
        (Http.post url (Http.jsonBody (encoder data)) decoder)

genericGet : String -> (Result Http.Error a -> Msg) -> Decode.Decoder a -> Cmd Msg
genericGet url m decoder = 
    Http.send
        m
        (Http.get url decoder)

renderCategory : Maybe RT.Category -> Html Msg
renderCategory category = 
    case category of 
        Nothing -> text "null"
        Just x -> text x.name


stepsView : List RT.Step -> Html Msg
stepsView steps = 
    div [] [
        div [Attributes.style "display" "inline-block"] [
            text "Something"
        ]
    ]


stepForm : RT.Step -> Html Msg
stepForm step =
    div [] [
    ]